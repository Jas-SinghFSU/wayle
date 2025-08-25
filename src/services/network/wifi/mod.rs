use std::{ops::Deref, sync::Arc};

use futures::stream::Stream;
use monitoring::WifiMonitor;
use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{
    AccessPointProxy, NetworkError, NetworkManagerProxy, NetworkStatus, SSID,
    core::{access_point::AccessPoint, device::wifi::DeviceWifi},
};
use crate::{services::common::Property, unwrap_bool, watch_all};

mod controls;
mod monitoring;

use controls::WifiControls;

/// Manages WiFi network connectivity and device state.
///
/// Provides high-level interface for WiFi operations including connection
/// management, access point scanning, and saved network handling. Wraps
/// the lower-level DeviceWifi D-Bus proxy with reactive properties for
/// state monitoring.
#[derive(Clone, Debug)]
pub struct Wifi {
    pub(crate) connection: Connection,
    /// The underlying WiFi device.
    pub device: DeviceWifi,

    /// Whether WiFi is enabled on the system.
    pub enabled: Property<bool>,
    /// Current WiFi connectivity status.
    pub connectivity: Property<NetworkStatus>,
    /// SSID of the currently connected network, if any.
    pub ssid: Property<Option<String>>,
    /// Signal strength of current connection (0-100).
    pub strength: Property<Option<u8>>,
    /// List of available access points.
    pub access_points: Property<Vec<Arc<AccessPoint>>>,
}

impl Deref for Wifi {
    type Target = DeviceWifi;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl PartialEq for Wifi {
    fn eq(&self, other: &Self) -> bool {
        self.device.object_path == other.device.object_path
    }
}

impl Wifi {
    /// Get a snapshot of the current WiFi state (no monitoring).
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::ObjectCreationFailed` if the WiFi device cannot be created
    pub async fn get(
        connection: &Connection,
        device_path: OwnedObjectPath,
    ) -> Result<Arc<Self>, NetworkError> {
        let device_arc = DeviceWifi::get(connection, device_path.clone())
            .await
            .map_err(|e| NetworkError::ObjectCreationFailed {
                object_type: "WiFi".to_string(),
                object_path: device_path.clone(),
                reason: e.to_string(),
            })?;
        let device = DeviceWifi::clone(&device_arc);

        let wifi = Self::from_device(connection, device).await?;
        Ok(Arc::new(wifi))
    }

    /// Get a live-updating WiFi instance (with monitoring).
    ///
    /// Fetches the device, current state and starts monitoring for updates.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::ObjectCreationFailed` if the WiFi device cannot be created
    /// or if monitoring fails to start
    pub async fn get_live(
        connection: &Connection,
        device_path: OwnedObjectPath,
        cancellation_token: CancellationToken,
    ) -> Result<Arc<Self>, NetworkError> {
        let device_arc =
            DeviceWifi::get_live(connection, device_path, cancellation_token.child_token()).await?;
        let device = DeviceWifi::clone(&device_arc);

        let wifi = Self::from_device(connection, device.clone()).await?;

        WifiMonitor::start(connection, &wifi, cancellation_token).await?;

        Ok(Arc::new(wifi))
    }

    /// Watch for any WiFi property changes.
    ///
    /// Emits whenever any WiFi property changes (enabled, connectivity, ssid, strength, or access points).
    pub fn watch(&self) -> impl Stream<Item = Wifi> + Send {
        watch_all!(self, enabled, connectivity, ssid, strength, access_points)
    }

    /// Enable or disable WiFi on the system.
    ///
    /// Controls the system-wide WiFi state through NetworkManager. When disabled,
    /// all WiFi connections are terminated.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if the operation fails.
    pub async fn set_enabled(&self, enabled: bool) -> Result<(), NetworkError> {
        WifiControls::set_enabled(&self.connection, enabled).await
    }

    /// Connect to a WiFi access point.
    ///
    /// Attempts to connect to the specified access point. NetworkManager will
    /// automatically check for existing connection profiles that match this network
    /// and reuse them if found, or create a new profile if needed.
    ///
    /// # Arguments
    ///
    /// * `ap_path` - D-Bus path of the access point to connect to
    /// * `password` - WiFi password (None for open networks)
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if the connection fails
    pub async fn connect(
        &self,
        ap_path: OwnedObjectPath,
        password: Option<String>,
    ) -> Result<(), NetworkError> {
        WifiControls::connect(
            &self.connection,
            &self.device.object_path,
            ap_path,
            password,
        )
        .await
    }

    /// Disconnect from the current WiFi network.
    ///
    /// Deactivates the current WiFi connection if there is one active.
    /// If no connection is active, this is a no-op.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if the disconnection fails
    pub async fn disconnect(&self) -> Result<(), NetworkError> {
        WifiControls::disconnect(&self.connection, &self.device.object_path).await
    }

    async fn from_device(
        connection: &Connection,
        device: DeviceWifi,
    ) -> Result<Self, NetworkError> {
        let nm_proxy = NetworkManagerProxy::new(connection).await?;

        let enabled_state = unwrap_bool!(nm_proxy.wireless_enabled().await);
        let device_state = &device.state.get();

        let active_ap_path = &device.active_access_point.get();
        let (ssid, strength) =
            match AccessPointProxy::new(connection, active_ap_path.to_string()).await {
                Ok(ap_proxy) => {
                    let ssid = ap_proxy
                        .ssid()
                        .await
                        .ok()
                        .map(|raw_ssid| SSID::new(raw_ssid).to_string());

                    let strength = ap_proxy.strength().await.ok();
                    (ssid, strength)
                }
                Err(_) => (None, None),
            };

        Ok(Self {
            connection: connection.clone(),
            device,
            enabled: Property::new(enabled_state),
            connectivity: Property::new(NetworkStatus::from_device_state(*device_state)),
            ssid: Property::new(ssid),
            strength: Property::new(strength),
            access_points: Property::new(vec![]),
        })
    }
}
