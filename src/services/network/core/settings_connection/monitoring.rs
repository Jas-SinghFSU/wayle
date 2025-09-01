use std::sync::Arc;

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use super::ConnectionSettings;
use crate::services::{
    network::{
        NMConnectionSettingsFlags, NetworkError,
        proxy::settings::connection::SettingsConnectionProxy,
    },
    traits::ModelMonitoring,
};

impl ModelMonitoring for ConnectionSettings {
    type Error = NetworkError;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let proxy = SettingsConnectionProxy::new(&self.connection, self.object_path.get())
            .await
            .map_err(NetworkError::DbusError)?;

        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(NetworkError::OperationFailed {
                operation: "start_monitoring",
                reason: String::from("A cancellation_token was not found."),
            });
        };

        let cancel_token = cancellation_token.clone();

        tokio::spawn(async move {
            monitor(self, proxy, cancel_token).await;
        });

        Ok(())
    }
}

async fn monitor(
    settings: Arc<ConnectionSettings>,
    proxy: SettingsConnectionProxy<'static>,
    cancellation_token: CancellationToken,
) {
    let mut unsaved_changed = proxy.receive_unsaved_changed().await;
    let mut flags_changed = proxy.receive_flags_changed().await;
    let mut filename_changed = proxy.receive_filename_changed().await;

    loop {
        tokio::select! {
            _ = cancellation_token.cancelled() => {
                debug!("ConnectionSettingsMonitor cancelled");
                return;
            }
            Some(change) = unsaved_changed.next() => {
                if let Ok(value) = change.get().await {
                    settings.unsaved.set(value);
                }
            }
            Some(change) = flags_changed.next() => {
                if let Ok(value) = change.get().await {
                    settings.flags.set(NMConnectionSettingsFlags::from_bits_truncate(value));
                }
            }
            Some(change) = filename_changed.next() => {
                if let Ok(value) = change.get().await {
                    settings.filename.set(value);
                }
            }
            else => {
                debug!("All property streams ended for SettingsConnection");
                break;
            }
        }
    }
}
