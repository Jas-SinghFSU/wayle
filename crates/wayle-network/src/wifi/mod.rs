mod controls;
mod monitoring;
mod types;

use std::sync::Arc;

use controls::WifiControls;
use futures::stream::Stream;
pub(crate) use types::{LiveWifiParams, WifiParams};
use wayle_common::{Property, unwrap_bool, watch_all};
use wayle_traits::{ModelMonitoring, Reactive};
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{
    core::{
        access_point::{AccessPoint, types::Ssid},
        device::wifi::{DeviceWifi, DeviceWifiParams, LiveDeviceWifiParams},
    },
    error::Error,
    proxy::{access_point::AccessPointProxy, manager::NetworkManagerProxy},
    types::states::NetworkStatus,
};

/// Manages WiFi network connectivity and device state.
///
/// Provides high-level interface for WiFi operations including connection
/// management, access point scanning, and saved network handling. Wraps
/// the lower-level DeviceWifi D-Bus proxy with reactive properties for
/// state monitoring.
#[derive(Clone, Debug)]
pub struct Wifi {
    /// The underlying WiFi device.
    pub device: DeviceWifi,

    /// Whether WiFi is enabled on the system.
    pub enabled: Property<bool>,
    /// Current WiFi connectivity status.
    pub connectivity: Property<NetworkStatus>,
    /// Ssid of the currently connected network, if any.
    pub ssid: Property<Option<String>>,
    /// Signal strength of current connection (0-100).
    pub strength: Property<Option<u8>>,
    /// List of available access points.
    pub access_points: Property<Vec<Arc<AccessPoint>>>,
}

impl PartialEq for Wifi {
    fn eq(&self, other: &Self) -> bool {
        self.device.core.object_path == other.device.core.object_path
    }
}

impl Reactive for Wifi {
    type Context<'a> = WifiParams<'a>;
    type LiveContext<'a> = LiveWifiParams<'a>;
    type Error = Error;

    async fn get(params: Self::Context<'_>) -> Result<Self, Self::Error> {
        let device = DeviceWifi::get(DeviceWifiParams {
            connection: params.connection,
            device_path: params.device_path.clone(),
        })
        .await
        .map_err(|e| Error::ObjectCreationFailed {
            object_type: String::from("WiFi"),
            object_path: params.device_path.clone(),
            source: e.into(),
        })?;
        Self::from_device(params.connection, device).await
    }

    async fn get_live(params: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let device_arc = DeviceWifi::get_live(LiveDeviceWifiParams {
            connection: params.connection,
            device_path: params.device_path,
            cancellation_token: params.cancellation_token,
        })
        .await?;
        let device = DeviceWifi::clone(&device_arc);

        let wifi = Self::from_device(params.connection, device.clone()).await?;
        let wifi = Arc::new(wifi);

        wifi.clone().start_monitoring().await?;

        Ok(wifi)
    }
}

impl Wifi {
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
    pub async fn set_enabled(&self, enabled: bool) -> Result<(), Error> {
        WifiControls::set_enabled(&self.device.core.connection, enabled).await
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
    ) -> Result<(), Error> {
        WifiControls::connect(
            &self.device.core.connection,
            &self.device.core.object_path,
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
    pub async fn disconnect(&self) -> Result<(), Error> {
        WifiControls::disconnect(&self.device.core.connection, &self.device.core.object_path).await
    }

    async fn from_device(connection: &Connection, device: DeviceWifi) -> Result<Self, Error> {
        let nm_proxy = NetworkManagerProxy::new(connection).await?;

        let enabled_state = unwrap_bool!(nm_proxy.wireless_enabled().await);
        let device_state = &device.core.state.get();

        let active_ap_path = &device.active_access_point.get();
        let (ssid, strength) =
            match AccessPointProxy::new(connection, active_ap_path.to_string()).await {
                Ok(ap_proxy) => {
                    let ssid = ap_proxy
                        .ssid()
                        .await
                        .ok()
                        .map(|raw_ssid| Ssid::new(raw_ssid).to_string());

                    let strength = ap_proxy.strength().await.ok();
                    (ssid, strength)
                }
                Err(_) => (None, None),
            };

        Ok(Self {
            device,
            enabled: Property::new(enabled_state),
            connectivity: Property::new(NetworkStatus::from_device_state(*device_state)),
            ssid: Property::new(ssid),
            strength: Property::new(strength),
            access_points: Property::new(vec![]),
        })
    }
}
