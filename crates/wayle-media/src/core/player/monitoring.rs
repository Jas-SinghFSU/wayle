use std::sync::{Arc, Weak};

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{debug, instrument};
use wayle_traits::ModelMonitoring;

use super::Player;
use crate::{
    error::Error,
    proxy::MediaPlayer2PlayerProxy,
    types::{LoopMode, PlaybackState, PlayerId, ShuffleMode, Volume},
};

impl ModelMonitoring for Player {
    type Error = Error;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(Error::Initialization(String::from(
                "missing cancellation token",
            )));
        };

        let cancel_token = cancellation_token.clone();
        let proxy = self.proxy.clone();
        let player_id = self.id.clone();
        let weak_self = Arc::downgrade(&self);

        debug!("Starting property monitoring for player: {}", player_id);

        tokio::spawn(async move {
            monitor_properties(player_id, weak_self, proxy, cancel_token).await;
        });

        Ok(())
    }
}

#[instrument(skip(weak_player, proxy))]
async fn monitor_properties(
    player_id: PlayerId,
    weak_player: Weak<Player>,
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
        let Some(player) = weak_player.upgrade() else {
            return;
        };

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
