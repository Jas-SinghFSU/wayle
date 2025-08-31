mod monitoring;

use std::{ops::Deref, sync::Arc};

use monitoring::DeviceWiredMonitor;
use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::Device;
use crate::{
    services::{
        common::Property,
        network::{DeviceProxy, NMDeviceType, NetworkError, wired_proxy::DeviceWiredProxy},
    },
    unwrap_string, unwrap_u32, unwrap_vec,
};

/// Speed in megabits per second (Mb/s).
pub type SpeedMbps = u32;

/// Wired-specific properties fetched from D-Bus
struct WiredProperties {
    perm_hw_address: String,
    speed: u32,
    s390_subchannels: Vec<String>,
}

/// Wired (Ethernet) network device.
///
/// Provides access to wired-specific properties like link speed and permanent
/// hardware address while inheriting all base device properties through Deref.
#[derive(Debug, Clone)]
pub struct DeviceWired {
    base: Device,

    /// Permanent hardware address of the device.
    pub perm_hw_address: Property<String>,

    /// Design speed of the device, in megabits/second (Mb/s).
    pub speed: Property<SpeedMbps>,

    /// Array of S/390 subchannels for S/390 or z/Architecture devices.
    pub s390_subchannels: Property<Vec<String>>,
}

impl Deref for DeviceWired {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DeviceWired {
    /// Get a snapshot of the current wired device state (no monitoring).
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::WrongObjectType` if device at path is not an ethernet device,
    /// `NetworkError::DbusError` if D-Bus operations fail.
    pub(crate) async fn get(
        connection: &Connection,
        device_path: OwnedObjectPath,
    ) -> Result<Arc<Self>, NetworkError> {
        let device = Self::from_path(connection, device_path).await?;
        Ok(Arc::new(device))
    }

    /// Get a live-updating wired device instance (with monitoring).
    ///
    /// Fetches current state and starts monitoring for updates.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `NetworkError::WrongObjectType` if device at path is not an ethernet device
    /// - `NetworkError::DbusError` if D-Bus operations fail
    pub(crate) async fn get_live(
        connection: &Connection,
        device_path: OwnedObjectPath,
        cancellation_token: CancellationToken,
    ) -> Result<Arc<Self>, NetworkError> {
        Self::verify_is_ethernet_device(connection, &device_path).await?;

        let base_device = Device::get_live(
            connection,
            device_path.clone(),
            cancellation_token.child_token(),
        )
        .await?;
        let base = Device::clone(&base_device);
        let wired_props = Self::fetch_wired_properties(connection, &device_path).await?;
        let device = Arc::new(Self::from_props(base, wired_props));

        DeviceWiredMonitor::start(device.clone(), connection, device_path, cancellation_token)
            .await?;

        Ok(device)
    }

    async fn verify_is_ethernet_device(
        connection: &Connection,
        device_path: &OwnedObjectPath,
    ) -> Result<(), NetworkError> {
        let device_proxy = DeviceProxy::new(connection, device_path)
            .await
            .map_err(NetworkError::DbusError)?;

        let device_type = device_proxy
            .device_type()
            .await
            .map_err(NetworkError::DbusError)?;

        if device_type != NMDeviceType::Ethernet as u32 {
            return Err(NetworkError::WrongObjectType {
                object_path: device_path.clone(),
                expected: "Ethernet device".to_string(),
                actual: format!("device type {device_type}"),
            });
        }

        Ok(())
    }

    async fn fetch_wired_properties(
        connection: &Connection,
        device_path: &OwnedObjectPath,
    ) -> Result<WiredProperties, NetworkError> {
        let wired_proxy = DeviceWiredProxy::new(connection, device_path)
            .await
            .map_err(NetworkError::DbusError)?;

        let (perm_hw_address, speed, s390_subchannels) = tokio::join!(
            wired_proxy.perm_hw_address(),
            wired_proxy.speed(),
            wired_proxy.s390_subchannels(),
        );

        Ok(WiredProperties {
            perm_hw_address: unwrap_string!(perm_hw_address, device_path),
            speed: unwrap_u32!(speed, device_path),
            s390_subchannels: unwrap_vec!(s390_subchannels, device_path),
        })
    }

    fn from_props(base: Device, props: WiredProperties) -> Self {
        Self {
            base,
            perm_hw_address: Property::new(props.perm_hw_address),
            speed: Property::new(props.speed),
            s390_subchannels: Property::new(props.s390_subchannels),
        }
    }

    async fn from_path(
        connection: &Connection,
        path: OwnedObjectPath,
    ) -> Result<Self, NetworkError> {
        Self::verify_is_ethernet_device(connection, &path).await?;

        let base = Device::from_path(connection, path.clone()).await?;
        let wired_props = Self::fetch_wired_properties(connection, &path).await?;

        Ok(Self::from_props(base, wired_props))
    }
}
