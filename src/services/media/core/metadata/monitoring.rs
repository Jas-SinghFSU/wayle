use std::sync::Arc;

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use super::TrackMetadata;
use crate::services::media::proxy::MediaPlayer2PlayerProxy;

/// Monitors D-Bus metadata properties and updates the reactive TrackMetadata model.
pub(super) struct TrackMetadataMonitor;

impl TrackMetadataMonitor {
    /// Start monitoring for metadata changes.
    ///
    /// Monitoring stops automatically when the TrackMetadata is dropped.
    pub(super) fn start(
        metadata: Arc<TrackMetadata>,
        proxy: MediaPlayer2PlayerProxy<'static>,
        cancellation_token: CancellationToken,
    ) {
        tokio::spawn(async move {
            Self::monitor(metadata, proxy, cancellation_token).await;
        });
    }

    async fn monitor(
        metadata: Arc<TrackMetadata>,
        proxy: MediaPlayer2PlayerProxy<'static>,
        cancellation_token: CancellationToken,
    ) {
        let mut metadata_changed = proxy.receive_metadata_changed().await;

        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("TrackMetadataMonitor cancelled");
                    return;
                }
                Some(change) = metadata_changed.next() => {
                    if let Ok(new_metadata) = change.get().await {
                        TrackMetadata::update_from_dbus(&metadata, new_metadata);
                    }
                }
                else => break
            }
        }

        debug!("Metadata monitoring ended");
    }
}
