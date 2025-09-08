use std::{collections::HashMap, sync::Arc};

use futures::StreamExt;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::{debug, instrument, warn};
use zbus::{Connection, fdo::DBusProxy};

use super::{core::player::Player, error::Error, types::PlayerId};
use crate::{
    core::state::RuntimeState,
    services::{
        common::Property,
        media::{core::player::LivePlayerParams, service::MediaService},
        traits::{Reactive, ServiceMonitoring},
    },
};

const MPRIS_BUS_PREFIX: &str = "org.mpris.MediaPlayer2.";

impl ServiceMonitoring for MediaService {
    type Error = Error;

    #[instrument(skip_all)]
    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        discover_existing_players(
            &self.connection,
            &self.players,
            &self.player_list,
            &self.active_player,
            &self.ignored_patterns,
            &self.cancellation_token,
        )
        .await?;

        if let Ok(Some(saved_player_id)) = RuntimeState::get_active_player().await {
            let players_map = self.players.read().await;
            if let Some(player_id) = players_map
                .keys()
                .find(|id| id.bus_name() == saved_player_id)
            {
                let pl = players_map
                    .get(player_id)
                    .cloned()
                    .ok_or_else(|| Error::PlayerNotFound(player_id.clone()))?;
                self.active_player.set(Some(pl));
                debug!("Restored active player from state: {}", saved_player_id);
            }
        }

        spawn_name_monitoring(
            &self.connection,
            Arc::clone(&self.players),
            self.player_list.clone(),
            self.active_player.clone(),
            self.ignored_patterns.clone(),
            self.cancellation_token.child_token(),
        );

        Ok(())
    }
}

async fn discover_existing_players(
    connection: &Connection,
    players: &Arc<RwLock<HashMap<PlayerId, Arc<Player>>>>,
    player_list: &Property<Vec<Arc<Player>>>,
    active_player: &Property<Option<Arc<Player>>>,
    ignored_patterns: &[String],
    cancellation_token: &CancellationToken,
) -> Result<(), Error> {
    let dbus_proxy = DBusProxy::new(connection)
        .await
        .map_err(|e| Error::InitializationFailed(format!("DBus proxy failed: {e}")))?;

    let names = dbus_proxy
        .list_names()
        .await
        .map_err(|e| Error::DbusError(e.into()))?;

    for name in names {
        if name.starts_with(MPRIS_BUS_PREFIX) && !should_ignore(&name, ignored_patterns) {
            let player_id = PlayerId::from_bus_name(&name);
            handle_player_added(
                connection,
                players,
                player_list,
                active_player,
                player_id,
                cancellation_token.child_token(),
            )
            .await;
        }
    }

    Ok(())
}

fn spawn_name_monitoring(
    connection: &Connection,
    players: Arc<RwLock<HashMap<PlayerId, Arc<Player>>>>,
    player_list: Property<Vec<Arc<Player>>>,
    active_player: Property<Option<Arc<Player>>>,
    ignored_patterns: Vec<String>,
    cancellation_token: CancellationToken,
) {
    let connection = connection.clone();

    tokio::spawn(async move {
        debug!("MprisMonitoring task spawned");
        let Ok(dbus_proxy) = DBusProxy::new(&connection).await else {
            warn!("Failed to create DBus proxy for name monitoring");
            return;
        };

        let Ok(mut name_owner_changed) = dbus_proxy.receive_name_owner_changed().await else {
            warn!("Failed to subscribe to NameOwnerChanged");
            return;
        };

        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("MprisMonitoring received cancellation signal, stopping all discovery");
                    return;
                }
                Some(signal) = name_owner_changed.next() => {
            let Ok(args) = signal.args() else { continue };

            if !args.name().starts_with(MPRIS_BUS_PREFIX) {
                continue;
            }

            let player_id = PlayerId::from_bus_name(args.name());

            let is_player_added = args.old_owner().is_none() && args.new_owner().is_some();
            let is_player_removed = args.old_owner().is_some() && args.new_owner().is_none();

            if is_player_added && !should_ignore(args.name(), &ignored_patterns) {
                handle_player_added(
                    &connection,
                    &players,
                    &player_list,
                    &active_player,
                    player_id.clone(),
                    cancellation_token.child_token(),
                )
                .await;
            } else if is_player_removed {
                handle_player_removed(&players, &player_list, &active_player, player_id)
                    .await;
            }
                }
                else => {
                    return;
                }
            }
        }
    });
}

async fn handle_player_added(
    connection: &Connection,
    players: &Arc<RwLock<HashMap<PlayerId, Arc<Player>>>>,
    player_list: &Property<Vec<Arc<Player>>>,
    active_player: &Property<Option<Arc<Player>>>,
    player_id: PlayerId,
    cancellation_token: CancellationToken,
) {
    match Player::get_live(LivePlayerParams {
        connection,
        player_id: player_id.clone(),
        cancellation_token: &cancellation_token,
    })
    .await
    {
        Ok(player) => {
            let mut players_map = players.write().await;
            players_map.insert(player_id.clone(), Arc::clone(&player));

            if active_player.get().is_none() {
                active_player.set(Some(player.clone()));
            }

            let mut current_list = player_list.get();
            current_list.push(player.clone());
            player_list.set(current_list);

            debug!("Player {} added", player_id);
        }
        Err(e) => {
            warn!("Failed to create player {}: {}", player_id, e);
        }
    }
}

async fn handle_player_removed(
    players: &Arc<RwLock<HashMap<PlayerId, Arc<Player>>>>,
    player_list: &Property<Vec<Arc<Player>>>,
    active_player: &Property<Option<Arc<Player>>>,
    player_id: PlayerId,
) {
    let mut players_map = players.write().await;
    players_map.remove(&player_id);

    if let Some(current_active) = active_player.get()
        && current_active.id == player_id
    {
        let new_active = players_map.values().next().cloned();
        active_player.set(new_active);
    }

    let mut current_players = player_list.get();
    current_players.retain(|player| {
        if player.id != player_id {
            return true;
        }

        if let Some(ref cancel_token) = player.cancellation_token {
            cancel_token.cancel();
        }

        false
    });

    player_list.set(current_players);

    debug!("Player {} removed", player_id);
}

fn should_ignore(bus_name: &str, ignored_patterns: &[String]) -> bool {
    ignored_patterns
        .iter()
        .any(|pattern| bus_name.contains(pattern))
}
