use std::sync::Arc;

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::DeviceWired;
use crate::services::network::{NetworkError, wired_proxy::DeviceWiredProxy};

/// Monitors D-Bus properties and updates the reactive DeviceWired model.
pub(crate) struct DeviceWiredMonitor;

impl DeviceWiredMonitor {
    pub(super) async fn start(
        device: Arc<DeviceWired>,
        connection: &Connection,
        path: OwnedObjectPath,
        cancellation_token: CancellationToken,
    ) -> Result<(), NetworkError> {
        let proxy = DeviceWiredProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        tokio::spawn(async move {
            Self::monitor(device, proxy, cancellation_token).await;
        });

        Ok(())
    }

    async fn monitor(
        device: Arc<DeviceWired>,
        proxy: DeviceWiredProxy<'static>,
        cancellation_token: CancellationToken,
    ) {
        let mut perm_hw_address_changed = proxy.receive_perm_hw_address_changed().await;
        let mut speed_changed = proxy.receive_speed_changed().await;
        let mut s390_subchannels_changed = proxy.receive_s390_subchannels_changed().await;

        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("DeviceWiredMonitor cancelled");
                    return;
                }
                Some(change) = perm_hw_address_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.perm_hw_address.set(value);
                    }
                }
                Some(change) = speed_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.speed.set(value);
                    }
                }
                Some(change) = s390_subchannels_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.s390_subchannels.set(value);
                    }
                }
                else => {
                    debug!("All property streams ended for DeviceWired");
                    break;
                }
            }
        }

        debug!("Property monitoring ended for DeviceWired");
    }
}
