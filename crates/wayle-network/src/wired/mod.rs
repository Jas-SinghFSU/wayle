mod monitoring;
mod types;

use std::sync::Arc;

pub(crate) use types::{LiveWiredParams, WiredParams};
use wayle_common::Property;
use wayle_traits::{ModelMonitoring, Reactive};

use super::{
    core::device::wired::{DeviceWired, DeviceWiredParams, LiveDeviceWiredParams},
    error::Error,
    types::states::NetworkStatus,
};

/// Wired (ethernet) device state. See [crate-level docs](crate) for usage.
#[derive(Clone, Debug)]
pub struct Wired {
    /// Underlying device properties.
    pub device: DeviceWired,
    /// Current connectivity status.
    pub connectivity: Property<NetworkStatus>,
}

impl PartialEq for Wired {
    fn eq(&self, other: &Self) -> bool {
        self.device.core.object_path == other.device.core.object_path
    }
}

impl Reactive for Wired {
    type Context<'a> = WiredParams<'a>;
    type LiveContext<'a> = LiveWiredParams<'a>;
    type Error = Error;

    async fn get(params: Self::Context<'_>) -> Result<Self, Self::Error> {
        let device = DeviceWired::get(DeviceWiredParams {
            connection: params.connection,
            device_path: params.device_path.clone(),
        })
        .await
        .map_err(|e| Error::ObjectCreationFailed {
            object_type: String::from("Wired"),
            object_path: params.device_path.clone(),
            source: e.into(),
        })?;

        Self::from_device(device).await
    }

    async fn get_live(params: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let device_arc = DeviceWired::get_live(LiveDeviceWiredParams {
            connection: params.connection,
            device_path: params.device_path,
            cancellation_token: params.cancellation_token,
        })
        .await?;
        let device = DeviceWired::clone(&device_arc);

        let wired = Self::from_device(device.clone()).await?;
        let wired = Arc::new(wired);

        wired.clone().start_monitoring().await?;

        Ok(wired)
    }
}

impl Wired {
    async fn from_device(device: DeviceWired) -> Result<Self, Error> {
        let device_state = &device.core.state.get();
        let connectivity = NetworkStatus::from_device_state(*device_state);

        Ok(Self {
            device,
            connectivity: Property::new(connectivity),
        })
    }
}
