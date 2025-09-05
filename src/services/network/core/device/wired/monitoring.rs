use std::sync::{Arc, Weak};

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use super::DeviceWired;
use crate::services::{
    network::{NetworkError, wired_proxy::DeviceWiredProxy},
    traits::ModelMonitoring,
};

impl ModelMonitoring for DeviceWired {
    type Error = NetworkError;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let base_arc = Arc::new(self.base.clone());
        base_arc.start_monitoring().await?;

        let proxy = DeviceWiredProxy::new(&self.connection, self.object_path.clone())
            .await
            .map_err(NetworkError::DbusError)?;

        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(NetworkError::OperationFailed {
                operation: "start_monitoring",
                reason: String::from("A cancellation_token was not found."),
            });
        };

        let cancel_token = cancellation_token.clone();
        let weak_self = Arc::downgrade(&self);

        tokio::spawn(async move {
            monitor_wired(weak_self, proxy, cancel_token).await;
        });

        Ok(())
    }
}

async fn monitor_wired(
    weak_device: Weak<DeviceWired>,
    proxy: DeviceWiredProxy<'static>,
    cancellation_token: CancellationToken,
) {
    let mut perm_hw_address_changed = proxy.receive_perm_hw_address_changed().await;
    let mut speed_changed = proxy.receive_speed_changed().await;
    let mut s390_subchannels_changed = proxy.receive_s390_subchannels_changed().await;

    loop {
        let Some(device) = weak_device.upgrade() else {
            return;
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                debug!("DeviceWired monitoring cancelled for {}", device.object_path);
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
