use std::sync::{Arc, Weak};

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use wayle_traits::ModelMonitoring;

use super::Wired;
use crate::{
    error::Error,
    proxy::devices::DeviceProxy,
    types::states::{NMDeviceState, NetworkStatus},
};

impl ModelMonitoring for Wired {
    type Error = Error;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let device_arc = Arc::new(self.device.clone());
        device_arc.start_monitoring().await?;

        let Some(ref cancellation_token) = self.device.core.cancellation_token else {
            return Err(Error::OperationFailed {
                operation: "start_monitoring",
                reason: String::from("A cancellation_token was not found."),
            });
        };

        let cancel_token = cancellation_token.clone();
        let weak_self = Arc::downgrade(&self);
        let device_proxy = DeviceProxy::new(
            &self.device.core.connection,
            self.device.core.object_path.clone(),
        )
        .await
        .map_err(Error::DbusError)?;

        tokio::spawn(async move {
            let _ = monitor_wired_connectivity(weak_self, device_proxy, cancel_token).await;
        });

        Ok(())
    }
}

async fn monitor_wired_connectivity(
    weak_wired: Weak<Wired>,
    proxy: DeviceProxy<'static>,
    cancellation_token: CancellationToken,
) -> Result<(), Error> {
    let mut connectivity_changed = proxy.receive_state_changed().await;

    loop {
        let Some(wired) = weak_wired.upgrade() else {
            return Ok(());
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                debug!("Wired monitoring cancelled for {}", wired.device.core.object_path);
                return Ok(());
            }
            Some(change) = connectivity_changed.next() => {
                if let Ok(new_connectivity) = change.get().await {
                    let device_state = NMDeviceState::from_u32(new_connectivity);
                    wired.connectivity.set(NetworkStatus::from_device_state(device_state));
                }
            }
            else => {
                break;
            }
        }
    }

    Ok(())
}
