use std::sync::Arc;

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{debug, instrument};

use super::Player;
use crate::services::media::{
    proxy::MediaPlayer2PlayerProxy,
    types::{LoopMode, PlaybackState, PlayerId, ShuffleMode, Volume},
};

/// Monitors D-Bus properties and updates the reactive Player model.
pub(super) struct PlayerMonitor;

impl PlayerMonitor {
    /// Start monitoring a player's D-Bus properties.
    ///
    /// Monitoring stops automatically when the Player is dropped.
    #[instrument(skip(player, proxy))]
    pub(super) fn start(
        player_id: PlayerId,
        player: Arc<Player>,
        proxy: MediaPlayer2PlayerProxy<'static>,
        cancellation_token: CancellationToken,
    ) {
        debug!("Starting property monitoring for player: {}", player_id);

        tokio::spawn(async move {
            Self::monitor_properties(player_id, player, proxy, cancellation_token).await;
        });
    }

    #[instrument(skip(player, proxy))]
    async fn monitor_properties(
        player_id: PlayerId,
        player: Arc<Player>,
        proxy: MediaPlayer2PlayerProxy<'static>,
        cancellation_token: CancellationToken,
    ) {
        let mut playback_status_changes = proxy.receive_playback_status_changed().await;
        let mut loop_status_changes = proxy.receive_loop_status_changed().await;
        let mut shuffle_changes = proxy.receive_shuffle_changed().await;
        let mut volume_changes = proxy.receive_volume_changed().await;
        let mut can_go_next_changes = proxy.receive_can_go_next_changed().await;
        let mut can_go_previous_changes = proxy.receive_can_go_previous_changed().await;
        let mut can_play_changes = proxy.receive_can_play_changed().await;
        let mut can_seek_changes = proxy.receive_can_seek_changed().await;

        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Player {} monitor received cancellation, stopping", player_id);
                    return;
                }
                Some(change) = playback_status_changes.next() => {
                    if let Ok(status) = change.get().await {
                        let state = PlaybackState::from(status.as_str());
                        player.playback_state.set(state);
                    }
                }

                Some(change) = loop_status_changes.next() => {
                    if let Ok(status) = change.get().await {
                        let mode = LoopMode::from(status.as_str());
                        player.loop_mode.set(mode);
                    }
                }

                Some(change) = shuffle_changes.next() => {
                    if let Ok(shuffle) = change.get().await {
                        let mode = ShuffleMode::from(shuffle);
                        player.shuffle_mode.set(mode);
                    }
                }

                Some(change) = volume_changes.next() => {
                    if let Ok(volume) = change.get().await {
                        player.volume.set(Volume::from(volume));
                    }
                }

                Some(change) = can_go_next_changes.next() => {
                    if let Ok(can_go_next) = change.get().await {
                        player.can_go_next.set(can_go_next);
                    }
                }

                Some(change) = can_go_previous_changes.next() => {
                    if let Ok(can_go_previous) = change.get().await {
                        player.can_go_previous.set(can_go_previous);
                    }
                }

                Some(change) = can_play_changes.next() => {
                    if let Ok(can_play) = change.get().await {
                        player.can_play.set(can_play);
                    }
                }

                Some(change) = can_seek_changes.next() => {
                    if let Ok(can_seek) = change.get().await {
                        player.can_seek.set(can_seek);
                    }
                }

                else => {
                    debug!("All property streams ended for player {}, exiting monitor", player_id);
                    break;
                }
            }
        }

        debug!(
            "Property monitoring fully terminated for player {}",
            player_id
        );
    }
}
