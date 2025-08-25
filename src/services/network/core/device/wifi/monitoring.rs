use std::sync::Arc;

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::DeviceWifi;
use crate::services::network::{
    NetworkError, proxy::devices::wireless::DeviceWirelessProxy, types::NM80211Mode,
};

/// Monitors D-Bus properties and updates the reactive DeviceWifi model.
pub(crate) struct DeviceWifiMonitor;

impl DeviceWifiMonitor {
    pub(super) async fn start(
        device: Arc<DeviceWifi>,
        connection: &Connection,
        path: OwnedObjectPath,
        cancellation_token: CancellationToken,
    ) -> Result<(), NetworkError> {
        let proxy = DeviceWirelessProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        tokio::spawn(async move {
            Self::monitor(device, proxy, cancellation_token).await;
        });

        Ok(())
    }

    #[allow(clippy::cognitive_complexity)]
    async fn monitor(
        device: Arc<DeviceWifi>,
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
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("DeviceWifiMonitor cancelled");
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
}
