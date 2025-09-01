use std::sync::Arc;

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use super::TrackMetadata;
use crate::services::{
    media::{MediaError, proxy::MediaPlayer2PlayerProxy},
    traits::ModelMonitoring,
};

impl ModelMonitoring for TrackMetadata {
    type Error = MediaError;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let Some(ref proxy) = self.proxy else {
            return Err(MediaError::InitializationFailed(String::from(
                "A proxy was not found.",
            )));
        };

        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(MediaError::InitializationFailed(String::from(
                "A cancellation_token was not found.",
            )));
        };

        let cancel_token = cancellation_token.clone();
        let proxy_clone = proxy.clone();

        tokio::spawn(async move {
            monitor(self, proxy_clone, cancel_token).await;
        });

        Ok(())
    }
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
