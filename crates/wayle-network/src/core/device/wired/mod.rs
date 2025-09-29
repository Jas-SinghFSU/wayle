pub(crate) mod monitoring;
pub(crate) mod types;

use std::sync::Arc;

use types::SpeedMbps;
pub(crate) use types::{DeviceWiredParams, LiveDeviceWiredParams, WiredProperties};
use wayle_common::{Property, unwrap_string, unwrap_u32, unwrap_vec};
use wayle_traits::{ModelMonitoring, Reactive};
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{Device, LiveDeviceParams};
use crate::{
    error::Error,
    proxy::devices::{DeviceProxy, wired::DeviceWiredProxy},
    types::device::NMDeviceType,
};

/// Wired (Ethernet) network device.
///
/// Provides access to wired-specific properties like link speed and permanent
/// hardware address while inheriting all base device properties through Deref.
#[derive(Debug, Clone)]
pub struct DeviceWired {
    /// The underlying NetworkManager device providing core network functionality.
    pub core: Device,

    /// Permanent hardware address of the device.
    pub perm_hw_address: Property<String>,

    /// Design speed of the device, in megabits/second (Mb/s).
    pub speed: Property<SpeedMbps>,

    /// Array of S/390 subchannels for S/390 or z/Architecture devices.
    pub s390_subchannels: Property<Vec<String>>,
}

impl Reactive for DeviceWired {
    type Context<'a> = DeviceWiredParams<'a>;
    type LiveContext<'a> = LiveDeviceWiredParams<'a>;
    type Error = Error;

    async fn get(params: Self::Context<'_>) -> Result<Self, Self::Error> {
        Self::from_path(params.connection, params.device_path).await
    }

    async fn get_live(params: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        Self::verify_is_ethernet_device(params.connection, &params.device_path).await?;

        let base_device = Device::get_live(LiveDeviceParams {
            connection: params.connection,
            object_path: params.device_path.clone(),
            cancellation_token: params.cancellation_token,
        })
        .await?;
        let base = Device::clone(&base_device);
        let wired_props =
            Self::fetch_wired_properties(params.connection, &params.device_path).await?;
        let device = Arc::new(Self::from_props(base, wired_props));

        device.clone().start_monitoring().await?;

        Ok(device)
    }
}

impl DeviceWired {
    async fn verify_is_ethernet_device(
        connection: &Connection,
        device_path: &OwnedObjectPath,
    ) -> Result<(), Error> {
        let device_proxy = DeviceProxy::new(connection, device_path)
            .await
            .map_err(Error::DbusError)?;

        let device_type = device_proxy.device_type().await.map_err(Error::DbusError)?;

        if device_type != NMDeviceType::Ethernet as u32 {
            return Err(Error::WrongObjectType {
                object_path: device_path.clone(),
                expected: String::from("Ethernet device"),
                actual: format!("device type {device_type}"),
            });
        }

        Ok(())
    }

    async fn fetch_wired_properties(
        connection: &Connection,
        device_path: &OwnedObjectPath,
    ) -> Result<WiredProperties, Error> {
        let wired_proxy = DeviceWiredProxy::new(connection, device_path)
            .await
            .map_err(Error::DbusError)?;

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

    fn from_props(core: Device, props: WiredProperties) -> Self {
        Self {
            core,
            perm_hw_address: Property::new(props.perm_hw_address),
            speed: Property::new(props.speed),
            s390_subchannels: Property::new(props.s390_subchannels),
        }
    }

    async fn from_path(connection: &Connection, path: OwnedObjectPath) -> Result<Self, Error> {
        Self::verify_is_ethernet_device(connection, &path).await?;

        let base = Device::from_path(connection, path.clone(), None).await?;
        let wired_props = Self::fetch_wired_properties(connection, &path).await?;

        Ok(Self::from_props(base, wired_props))
    }
}
