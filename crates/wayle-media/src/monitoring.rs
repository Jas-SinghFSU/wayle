use std::{collections::HashMap, sync::Arc};

use futures::StreamExt;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::{debug, instrument, warn};
use wayle_common::{Property, RuntimeState};
use wayle_traits::{Reactive, ServiceMonitoring};
use zbus::{Connection, fdo::DBusProxy};

use super::{core::player::Player, error::Error, types::PlayerId};
use crate::{
    core::player::LivePlayerParams,
    selection::{SelectionContext, select_best_player},
    service::MediaService,
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
            &self.priority_patterns,
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
            self.priority_patterns.clone(),
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
    priority_patterns: &[String],
    cancellation_token: &CancellationToken,
) -> Result<(), Error> {
    let dbus_proxy = DBusProxy::new(connection)
        .await
        .map_err(|e| Error::Initialization(format!("d-bus proxy: {e}")))?;

    let names = dbus_proxy
        .list_names()
        .await
        .map_err(|e| Error::Dbus(e.into()))?;

    for name in names {
        if name.starts_with(MPRIS_BUS_PREFIX) && !should_ignore(&name, ignored_patterns) {
            let player_id = PlayerId::from_bus_name(&name);
            handle_player_added(
                connection,
                players,
                player_list,
                active_player,
                priority_patterns,
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
    priority_patterns: Vec<String>,
    cancellation_token: CancellationToken,
) {
    let connection = connection.clone();

    tokio::spawn(async move {
        debug!("MprisMonitoring task spawned");
        let Ok(dbus_proxy) = DBusProxy::new(&connection).await else {
            warn!("cannot create DBus proxy for name monitoring");
            return;
        };

        let Ok(mut name_owner_changed) = dbus_proxy.receive_name_owner_changed().await else {
            warn!("cannot subscribe to NameOwnerChanged");
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
                    &priority_patterns,
                    player_id.clone(),
                    cancellation_token.child_token(),
                )
                .await;
            } else if is_player_removed {
                handle_player_removed(&players, &player_list, &active_player, &priority_patterns, player_id)
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
    priority_patterns: &[String],
    player_id: PlayerId,
    cancellation_token: CancellationToken,
) {
    let player = match Player::get_live(LivePlayerParams {
        connection,
        player_id: player_id.clone(),
        cancellation_token: &cancellation_token,
    })
    .await
    {
        Ok(player) => player,
        Err(e) => {
            warn!(error = %e, player_id = %player_id, "cannot create player");
            return;
        }
    };

    let mut players_map = players.write().await;
    players_map.insert(player_id.clone(), Arc::clone(&player));

    let mut current_list = player_list.get();
    current_list.push(player.clone());
    player_list.set(current_list.clone());

    let best = select_best_player(&SelectionContext {
        players: &current_list,
        priority_patterns,
    });
    active_player.set(best);

    debug!("Player {} added", player_id);
}

async fn handle_player_removed(
    players: &Arc<RwLock<HashMap<PlayerId, Arc<Player>>>>,
    player_list: &Property<Vec<Arc<Player>>>,
    active_player: &Property<Option<Arc<Player>>>,
    priority_patterns: &[String],
    player_id: PlayerId,
) {
    let mut players_map = players.write().await;
    players_map.remove(&player_id);

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

    player_list.set(current_players.clone());

    let needs_reselection = active_player
        .get()
        .is_some_and(|current| current.id == player_id);

    if needs_reselection {
        let best = select_best_player(&SelectionContext {
            players: &current_players,
            priority_patterns,
        });
        active_player.set(best);
    }

    debug!("Player {} removed", player_id);
}

fn should_ignore(bus_name: &str, ignored_patterns: &[String]) -> bool {
    ignored_patterns
        .iter()
        .any(|pattern| bus_name.contains(pattern))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_ignore_returns_true_when_pattern_matches() {
        let patterns = vec![String::from("spotify"), String::from("chrome")];
        let bus_name = "org.mpris.MediaPlayer2.spotify";

        assert!(should_ignore(bus_name, &patterns));
    }

    #[test]
    fn should_ignore_returns_false_when_no_patterns_match() {
        let patterns = vec![String::from("spotify"), String::from("chrome")];
        let bus_name = "org.mpris.MediaPlayer2.vlc";

        assert!(!should_ignore(bus_name, &patterns));
    }

    #[test]
    fn should_ignore_with_empty_patterns_returns_false() {
        let patterns = vec![];
        let bus_name = "org.mpris.MediaPlayer2.spotify";

        assert!(!should_ignore(bus_name, &patterns));
    }

    #[test]
    fn should_ignore_matches_substring_in_bus_name() {
        let patterns = vec![String::from("chromium")];
        let bus_name = "org.mpris.MediaPlayer2.chromium.instance123";

        assert!(should_ignore(bus_name, &patterns));
    }
}
