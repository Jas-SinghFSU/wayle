use std::sync::{Arc, Weak};

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use wayle_traits::ModelMonitoring;

use super::DeviceWifi;
use crate::{
    error::Error, proxy::devices::wireless::DeviceWirelessProxy, types::wifi::NM80211Mode,
};

impl ModelMonitoring for DeviceWifi {
    type Error = Error;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let base_arc = Arc::new(self.core.clone());
        base_arc.start_monitoring().await?;

        let proxy = DeviceWirelessProxy::new(&self.core.connection, self.core.object_path.clone())
            .await
            .map_err(Error::DbusError)?;

        let Some(ref cancellation_token) = self.core.cancellation_token else {
            return Err(Error::MissingCancellationToken);
        };

        let cancel_token = cancellation_token.clone();
        let weak_self = Arc::downgrade(&self);

        tokio::spawn(async move {
            monitor_wifi(weak_self, proxy, cancel_token).await;
        });

        Ok(())
    }
}

#[allow(clippy::cognitive_complexity)]
async fn monitor_wifi(
    weak_device: Weak<DeviceWifi>,
    proxy: DeviceWirelessProxy<'static>,
    cancellation_token: CancellationToken,
) {
    let mut perm_hw_address_changed = proxy.receive_perm_hw_address_changed().await;
    let mut mode_changed = proxy.receive_mode_changed().await;
    let mut bitrate_changed = proxy.receive_bitrate_changed().await;
    let mut access_points_changed = proxy.receive_access_points_changed().await;
    let mut active_access_point_changed = proxy.receive_active_access_point_changed().await;
    let mut wireless_capabilities_changed = proxy.receive_wireless_capabilities_changed().await;
    let mut last_scan_changed = proxy.receive_last_scan_changed().await;

    loop {
        let Some(device) = weak_device.upgrade() else {
            return;
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                debug!("DeviceWifi monitoring cancelled for {}", device.core.object_path);
                return;
            }
            Some(change) = perm_hw_address_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.perm_hw_address.set(value);
                }
            }
            Some(change) = mode_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.mode.set(NM80211Mode::from_u32(value));
                }
            }
            Some(change) = bitrate_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.bitrate.set(value);
                }
            }
            Some(change) = access_points_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.access_points.set(value);
                }
            }
            Some(change) = active_access_point_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.active_access_point.set(value);
                }
            }
            Some(change) = wireless_capabilities_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.wireless_capabilities.set(value);
                }
            }
            Some(change) = last_scan_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.last_scan.set(value);
                }
            }
            else => {
                debug!("All property streams ended for DeviceWifi");
                break;
            }
        }
    }

    debug!("Property monitoring ended for DeviceWifi");
}
