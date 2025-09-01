use std::sync::Arc;

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use super::Wired;
use crate::services::{
    network::{DeviceProxy, NMDeviceState, NetworkError, NetworkStatus},
    traits::ModelMonitoring,
};

impl ModelMonitoring for Wired {
    type Error = NetworkError;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let device_arc = Arc::new(self.device.clone());
        device_arc.start_monitoring().await?;

        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(NetworkError::OperationFailed {
                operation: "start_monitoring",
                reason: "A cancellation_token was not found.".to_string(),
            });
        };

        let cancel_token = cancellation_token.clone();

        tokio::spawn(async move {
            let _ = monitor_wired_connectivity(self, cancel_token).await;
        });

        Ok(())
    }
}

async fn monitor_wired_connectivity(
    wired: Arc<Wired>,
    cancellation_token: CancellationToken,
) -> Result<(), NetworkError> {
    let connectivity_prop = wired.connectivity.clone();
    let device_path = wired.device.object_path.clone();

    let device_proxy = DeviceProxy::new(&wired.connection, device_path)
        .await
        .map_err(NetworkError::DbusError)?;

    let mut connectivity_changed = device_proxy.receive_state_changed().await;

    loop {
        tokio::select! {
            _ = cancellation_token.cancelled() => {
                debug!("Wired monitoring cancelled for {}", wired.device.object_path);
                return Ok(());
            }
            Some(change) = connectivity_changed.next() => {
                if let Ok(new_connectivity) = change.get().await {
                    let device_state = NMDeviceState::from_u32(new_connectivity);
                    connectivity_prop.set(NetworkStatus::from_device_state(device_state));
                }
            }
            else => {
                break;
            }
        }
    }

    Ok(())
}
