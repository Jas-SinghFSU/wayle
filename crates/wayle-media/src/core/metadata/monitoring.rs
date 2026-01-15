use std::sync::{Arc, Weak};

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use wayle_traits::ModelMonitoring;

use super::TrackMetadata;
use crate::{error::Error, proxy::MediaPlayer2PlayerProxy};

impl ModelMonitoring for TrackMetadata {
    type Error = Error;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let Some(ref proxy) = self.proxy else {
            return Err(Error::Initialization(String::from("missing proxy")));
        };

        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(Error::Initialization(String::from(
                "missing cancellation token",
            )));
        };

        let cancel_token = cancellation_token.clone();
        let proxy_clone = proxy.clone();
        let weak_self = Arc::downgrade(&self);

        tokio::spawn(async move {
            monitor(weak_self, proxy_clone, cancel_token).await;
        });

        Ok(())
    }
}

async fn monitor(
    weak_metadata: Weak<TrackMetadata>,
    proxy: MediaPlayer2PlayerProxy<'static>,
    cancellation_token: CancellationToken,
) {
    let mut metadata_changed = proxy.receive_metadata_changed().await;

    loop {
        let Some(metadata) = weak_metadata.upgrade() else {
            return;
        };

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
