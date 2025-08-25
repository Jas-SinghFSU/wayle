use std::{collections::HashMap, sync::Arc};

use futures::StreamExt;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::{debug, instrument, warn};
use zbus::{Connection, fdo::DBusProxy};

use crate::{
    core::state::RuntimeState,
    services::{
        common::Property,
        media::{MediaError, PlayerId, core::Player},
    },
};

const MPRIS_BUS_PREFIX: &str = "org.mpris.MediaPlayer2.";

/// Handles MPRIS player discovery monitoring.
///
/// Similar to NetworkManager's device discovery, this monitors for
/// MPRIS players appearing and disappearing on D-Bus.
pub(crate) struct MprisMonitoring;

impl MprisMonitoring {
    /// Start monitoring for MPRIS players.
    ///
    /// Discovers existing players and monitors for new players being added/removed.
    #[instrument(skip_all)]
    pub async fn start(
        connection: &Connection,
        players: Arc<RwLock<HashMap<PlayerId, Arc<Player>>>>,
        player_list: Property<Vec<Arc<Player>>>,
        active_player: Property<Option<Arc<Player>>>,
        ignored_patterns: Vec<String>,
        cancellation_token: CancellationToken,
    ) -> Result<(), MediaError> {
        Self::discover_existing_players(
            connection,
            &players,
            &player_list,
            &active_player,
            &ignored_patterns,
            &cancellation_token,
        )
        .await?;

        if let Ok(Some(saved_player_id)) = RuntimeState::get_active_player().await {
            let players_map = players.read().await;
            if let Some(player_id) = players_map
                .keys()
                .find(|id| id.bus_name() == saved_player_id)
            {
                let pl = players_map
                    .get(player_id)
                    .cloned()
                    .ok_or_else(|| MediaError::PlayerNotFound(player_id.clone()))?;
                active_player.set(Some(pl));
                debug!("Restored active player from state: {}", saved_player_id);
            }
        }

        Self::spawn_name_monitoring(
            connection,
            players,
            player_list,
            active_player,
            ignored_patterns,
            cancellation_token,
        );

        Ok(())
    }

    async fn discover_existing_players(
        connection: &Connection,
        players: &Arc<RwLock<HashMap<PlayerId, Arc<Player>>>>,
        player_list: &Property<Vec<Arc<Player>>>,
        active_player: &Property<Option<Arc<Player>>>,
        ignored_patterns: &[String],
        cancellation_token: &CancellationToken,
    ) -> Result<(), MediaError> {
        let dbus_proxy = DBusProxy::new(connection)
            .await
            .map_err(|e| MediaError::InitializationFailed(format!("DBus proxy failed: {e}")))?;

        let names = dbus_proxy
            .list_names()
            .await
            .map_err(|e| MediaError::DbusError(e.into()))?;

        for name in names {
            if name.starts_with(MPRIS_BUS_PREFIX) && !Self::should_ignore(&name, ignored_patterns) {
                let player_id = PlayerId::from_bus_name(&name);
                Self::handle_player_added(
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

                if is_player_added && !Self::should_ignore(args.name(), &ignored_patterns) {
                    Self::handle_player_added(
                        &connection,
                        &players,
                        &player_list,
                        &active_player,
                        player_id.clone(),
                        cancellation_token.child_token(),
                    )
                    .await;
                } else if is_player_removed {
                    Self::handle_player_removed(&players, &player_list, &active_player, player_id)
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
        match Player::get_live(connection, player_id.clone(), cancellation_token).await {
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

        let current_list = player_list.get();
        let updated_list: Vec<Arc<Player>> = current_list
            .into_iter()
            .filter(|player| player.id != player_id)
            .collect();
        player_list.set(updated_list);

        debug!("Player {} removed", player_id);
    }

    fn should_ignore(bus_name: &str, ignored_patterns: &[String]) -> bool {
        ignored_patterns
            .iter()
            .any(|pattern| bus_name.contains(pattern))
    }
}
