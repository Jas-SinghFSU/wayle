mod controls;
mod monitoring;

use std::{collections::HashMap, ops::Deref, sync::Arc};

use controls::DeviceWifiControls;
use monitoring::DeviceWifiMonitor;
use tokio_util::sync::CancellationToken;
use tracing::warn;
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::Device;
use crate::{
    services::{
        common::Property,
        network::{
            NMDeviceType, NetworkError,
            proxy::devices::{DeviceProxy, wireless::DeviceWirelessProxy},
            types::NM80211Mode,
        },
    },
    unwrap_i64_or, unwrap_path_or, unwrap_string, unwrap_u32, unwrap_vec,
};

/// Bitrate in kilobits per second (Kb/s).
pub type BitrateKbps = u32;

/// Timestamp in CLOCK_BOOTTIME milliseconds.
pub type BootTimeMs = i64;

/// Wireless device capabilities.
pub type WirelessCapabilities = u32;

/// WiFi-specific properties fetched from D-Bus
struct WifiProperties {
    perm_hw_address: String,
    mode: u32,
    bitrate: u32,
    access_points: Vec<OwnedObjectPath>,
    active_access_point: OwnedObjectPath,
    wireless_capabilities: u32,
    last_scan: i64,
}

/// Wireless (Wi-Fi) network device.
///
/// Provides access to wireless-specific properties like access points, signal
/// strength, and scanning while inheriting all base device properties through Deref.
#[derive(Debug, Clone)]
pub struct DeviceWifi {
    base: Device,

    /// Permanent hardware address of the device.
    pub perm_hw_address: Property<String>,

    /// The operating mode of the wireless device.
    pub mode: Property<NM80211Mode>,

    /// The bit rate currently used by the wireless device, in kilobits/second (Kb/s).
    pub bitrate: Property<BitrateKbps>,

    /// List of object paths of access points visible to this wireless device.
    pub access_points: Property<Vec<OwnedObjectPath>>,

    /// Object path of the access point currently used by the wireless device.
    pub active_access_point: Property<OwnedObjectPath>,

    /// The capabilities of the wireless device.
    pub wireless_capabilities: Property<WirelessCapabilities>,

    /// The timestamp (in CLOCK_BOOTTIME milliseconds) for the last finished network scan.
    /// A value of -1 means the device never scanned for access points.
    pub last_scan: Property<BootTimeMs>,
}

impl Deref for DeviceWifi {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DeviceWifi {
    /// Get a snapshot of the current WiFi device state (no monitoring).
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::WrongObjectType` if device at path is not a WiFi device,
    /// `NetworkError::ObjectCreationFailed` if failed to create base device, or
    /// `NetworkError::DbusError` if D-Bus operations fail.
    pub(crate) async fn get(
        connection: &Connection,
        device_path: OwnedObjectPath,
    ) -> Result<Arc<Self>, NetworkError> {
        let device = Self::from_path(connection, device_path).await?;
        Ok(Arc::new(device))
    }

    /// Get a live-updating WiFi device instance (with monitoring).
    ///
    /// Fetches current state and starts monitoring for updates.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `NetworkError::WrongObjectType` if device at path is not a WiFi device
    /// - `NetworkError::ObjectCreationFailed` if failed to create base device
    /// - `NetworkError::DbusError` if D-Bus operations fail
    pub(crate) async fn get_live(
        connection: &Connection,
        device_path: OwnedObjectPath,
        cancellation_token: CancellationToken,
    ) -> Result<Arc<Self>, NetworkError> {
        Self::verify_is_wifi_device(connection, &device_path).await?;

        let base_arc = Device::get_live(
            connection,
            device_path.clone(),
            cancellation_token.child_token(),
        )
        .await?;
        let base = Device::clone(&base_arc);

        let wifi_props = Self::fetch_wifi_properties(connection, &device_path).await?;
        let device = Arc::new(Self::from_props(base, wifi_props));

        DeviceWifiMonitor::start(device.clone(), connection, device_path, cancellation_token)
            .await?;

        Ok(device)
    }

    /// Request a scan for available access points.
    ///
    /// Triggers NetworkManager to scan for nearby WiFi networks. The scan runs
    /// asynchronously and results will be reflected in the `access_points` property
    /// when complete. The `last_scan` timestamp will update when the scan finishes.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::DbusError` if D-Bus proxy creation fails or
    /// `NetworkError::OperationFailed` if the scan request fails.
    pub async fn request_scan(&self) -> Result<(), NetworkError> {
        DeviceWifiControls::request_scan(&self.connection, &self.object_path, HashMap::new()).await
    }

    /// Get the list of all access points visible to this device, including hidden ones.
    ///
    /// # Errors
    /// Returns error if the D-Bus operation fails.
    pub async fn get_all_access_points(&self) -> Result<Vec<OwnedObjectPath>, NetworkError> {
        DeviceWifiControls::get_all_access_points(&self.connection, &self.object_path).await
    }

    async fn verify_is_wifi_device(
        connection: &Connection,
        object_path: &OwnedObjectPath,
    ) -> Result<(), NetworkError> {
        let device_proxy = DeviceProxy::new(connection, object_path.clone())
            .await
            .map_err(NetworkError::DbusError)?;

        let device_type = device_proxy
            .device_type()
            .await
            .map_err(NetworkError::DbusError)?;

        if device_type != NMDeviceType::Wifi as u32 {
            return Err(NetworkError::WrongObjectType {
                object_path: object_path.clone(),
                expected: "WiFi device".to_string(),
                actual: format!("device type {device_type}"),
            });
        }

        Ok(())
    }

    async fn fetch_wifi_properties(
        connection: &Connection,
        device_path: &OwnedObjectPath,
    ) -> Result<WifiProperties, NetworkError> {
        let wifi_proxy = DeviceWirelessProxy::new(connection, device_path.clone())
            .await
            .map_err(NetworkError::DbusError)?;

        let (
            perm_hw_address,
            mode,
            bitrate,
            access_points,
            active_access_point,
            wireless_capabilities,
            last_scan,
        ) = tokio::join!(
            wifi_proxy.perm_hw_address(),
            wifi_proxy.mode(),
            wifi_proxy.bitrate(),
            wifi_proxy.access_points(),
            wifi_proxy.active_access_point(),
            wifi_proxy.wireless_capabilities(),
            wifi_proxy.last_scan(),
        );

        Ok(WifiProperties {
            perm_hw_address: unwrap_string!(perm_hw_address, device_path),
            mode: unwrap_u32!(mode, device_path),
            bitrate: unwrap_u32!(bitrate, device_path),
            access_points: unwrap_vec!(access_points, device_path),
            active_access_point: unwrap_path_or!(
                active_access_point,
                device_path,
                OwnedObjectPath::default()
            ),
            wireless_capabilities: unwrap_u32!(wireless_capabilities, device_path),
            last_scan: unwrap_i64_or!(last_scan, -1, device_path),
        })
    }

    fn from_props(base: Device, props: WifiProperties) -> Self {
        Self {
            base,
            perm_hw_address: Property::new(props.perm_hw_address),
            mode: Property::new(NM80211Mode::from_u32(props.mode)),
            bitrate: Property::new(props.bitrate),
            access_points: Property::new(props.access_points),
            active_access_point: Property::new(props.active_access_point),
            wireless_capabilities: Property::new(props.wireless_capabilities),
            last_scan: Property::new(props.last_scan),
        }
    }

    async fn from_path(
        connection: &Connection,
        object_path: OwnedObjectPath,
    ) -> Result<Self, NetworkError> {
        let device_proxy = DeviceProxy::new(connection, object_path.clone()).await?;

        let device_type = device_proxy.device_type().await?;
        if device_type != NMDeviceType::Wifi as u32 {
            warn!(
                "Device at {object_path} is not a wifi device, got type: {} ({:?})",
                device_type,
                NMDeviceType::from_u32(device_type)
            );
            return Err(NetworkError::WrongObjectType {
                object_path: object_path.clone(),
                expected: "WiFi device".to_string(),
                actual: format!("{:?}", NMDeviceType::from_u32(device_type)),
            });
        }

        let wifi_proxy = DeviceWirelessProxy::new(connection, object_path.clone()).await?;

        let base = match Device::from_path(connection, object_path.clone()).await {
            Ok(base) => base,
            Err(e) => {
                warn!("Failed to create base Device for {}", object_path);
                return Err(NetworkError::ObjectCreationFailed {
                    object_type: "Device".to_string(),
                    object_path: object_path.clone(),
                    reason: e.to_string(),
                });
            }
        };

        let (
            perm_hw_address,
            mode,
            bitrate,
            access_points,
            active_access_point,
            wireless_capabilities,
            last_scan,
        ) = tokio::join!(
            wifi_proxy.perm_hw_address(),
            wifi_proxy.mode(),
            wifi_proxy.bitrate(),
            wifi_proxy.access_points(),
            wifi_proxy.active_access_point(),
            wifi_proxy.wireless_capabilities(),
            wifi_proxy.last_scan(),
        );

        let device = Self {
            base,
            perm_hw_address: Property::new(unwrap_string!(perm_hw_address)),
            mode: Property::new(NM80211Mode::from_u32(unwrap_u32!(mode))),
            bitrate: Property::new(unwrap_u32!(bitrate)),
            access_points: Property::new(unwrap_vec!(access_points)),
            active_access_point: Property::new(unwrap_path_or!(
                active_access_point,
                OwnedObjectPath::default()
            )),
            wireless_capabilities: Property::new(unwrap_u32!(wireless_capabilities)),
            last_scan: Property::new(unwrap_i64_or!(last_scan, -1)),
        };

        Ok(device)
    }
}
