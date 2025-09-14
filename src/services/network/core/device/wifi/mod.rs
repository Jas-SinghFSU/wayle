pub(crate) mod controls;
pub(crate) mod monitoring;
/// WiFi device types
pub mod types;

use std::{collections::HashMap, sync::Arc};

use controls::DeviceWifiControls;
use tracing::warn;
use types::{BitrateKbps, BootTimeMs, WifiProperties, WirelessCapabilities};
pub(crate) use types::{DeviceWifiParams, LiveDeviceWifiParams};
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{Device, LiveDeviceParams};
use crate::{
    services::{
        common::Property,
        network::{
            error::Error,
            proxy::devices::{DeviceProxy, wireless::DeviceWirelessProxy},
            types::{device::NMDeviceType, wifi::NM80211Mode},
        },
        traits::{ModelMonitoring, Reactive},
    },
    unwrap_i64_or, unwrap_path_or, unwrap_string, unwrap_u32, unwrap_vec,
};

/// Wireless (Wi-Fi) network device.
///
/// Provides access to wireless-specific properties like access points, signal
/// strength, and scanning while inheriting all base device properties through Deref.
#[derive(Debug, Clone)]
pub struct DeviceWifi {
    /// The underlying NetworkManager device providing core network functionality.
    pub core: Device,

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

impl Reactive for DeviceWifi {
    type Context<'a> = DeviceWifiParams<'a>;
    type LiveContext<'a> = LiveDeviceWifiParams<'a>;
    type Error = Error;

    async fn get(params: Self::Context<'_>) -> Result<Self, Self::Error> {
        Self::from_path(params.connection, params.device_path).await
    }

    async fn get_live(params: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        Self::verify_is_wifi_device(params.connection, &params.device_path).await?;

        let base_arc = Device::get_live(LiveDeviceParams {
            connection: params.connection,
            object_path: params.device_path.clone(),
            cancellation_token: params.cancellation_token,
        })
        .await?;
        let base = Device::clone(&base_arc);

        let wifi_props =
            Self::fetch_wifi_properties(params.connection, &params.device_path).await?;
        let device = Arc::new(Self::from_props(base, wifi_props));

        device.clone().start_monitoring().await?;

        Ok(device)
    }
}

impl DeviceWifi {
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
    pub async fn request_scan(&self) -> Result<(), Error> {
        DeviceWifiControls::request_scan(
            &self.core.connection,
            &self.core.object_path,
            HashMap::new(),
        )
        .await
    }

    /// Get the list of all access points visible to this device, including hidden ones.
    ///
    /// # Errors
    /// Returns error if the D-Bus operation fails.
    pub async fn get_all_access_points(&self) -> Result<Vec<OwnedObjectPath>, Error> {
        DeviceWifiControls::get_all_access_points(&self.core.connection, &self.core.object_path)
            .await
    }

    async fn verify_is_wifi_device(
        connection: &Connection,
        object_path: &OwnedObjectPath,
    ) -> Result<(), Error> {
        let device_proxy = DeviceProxy::new(connection, object_path)
            .await
            .map_err(Error::DbusError)?;

        let device_type = device_proxy.device_type().await.map_err(Error::DbusError)?;

        if device_type != NMDeviceType::Wifi as u32 {
            return Err(Error::WrongObjectType {
                object_path: object_path.clone(),
                expected: String::from("WiFi device"),
                actual: format!("device type {device_type}"),
            });
        }

        Ok(())
    }

    async fn fetch_wifi_properties(
        connection: &Connection,
        device_path: &OwnedObjectPath,
    ) -> Result<WifiProperties, Error> {
        let wifi_proxy = DeviceWirelessProxy::new(connection, device_path)
            .await
            .map_err(Error::DbusError)?;

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

    fn from_props(core: Device, props: WifiProperties) -> Self {
        Self {
            core,
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
    ) -> Result<Self, Error> {
        let device_proxy = DeviceProxy::new(connection, &object_path).await?;

        let device_type = device_proxy.device_type().await?;
        if device_type != NMDeviceType::Wifi as u32 {
            warn!(
                "Device at {object_path} is not a wifi device, got type: {} ({:?})",
                device_type,
                NMDeviceType::from_u32(device_type)
            );
            return Err(Error::WrongObjectType {
                object_path: object_path.clone(),
                expected: String::from("WiFi device"),
                actual: format!("{:?}", NMDeviceType::from_u32(device_type)),
            });
        }

        let wifi_proxy = DeviceWirelessProxy::new(connection, &object_path).await?;

        let base = match Device::from_path(connection, object_path.clone(), None).await {
            Ok(base) => base,
            Err(e) => {
                warn!("Failed to create base Device for {}", object_path);
                return Err(Error::ObjectCreationFailed {
                    object_type: String::from("Device"),
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
            core: base,
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
