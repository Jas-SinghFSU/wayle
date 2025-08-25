use std::sync::Arc;

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::ConnectionSettings;
use crate::services::network::{
    NMConnectionSettingsFlags, NetworkError, proxy::settings::connection::SettingsConnectionProxy,
};

/// Monitors D-Bus properties and updates the reactive SettingsConnection model.
pub(crate) struct ConnectionSettingsMonitor;

impl ConnectionSettingsMonitor {
    pub(super) async fn start(
        settings: Arc<ConnectionSettings>,
        connection: &Connection,
        path: OwnedObjectPath,
        cancellation_token: CancellationToken,
    ) -> Result<(), NetworkError> {
        let proxy = SettingsConnectionProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        tokio::spawn(async move {
            Self::monitor(settings, proxy, cancellation_token).await;
        });

        Ok(())
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
}
