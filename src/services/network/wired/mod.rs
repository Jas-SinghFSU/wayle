mod monitoring;
mod types;

use std::sync::Arc;

pub(crate) use types::{LiveWiredParams, WiredParams};

use super::{
    core::device::wired::{DeviceWired, DeviceWiredParams, LiveDeviceWiredParams},
    error::Error,
    types::states::NetworkStatus,
};
use crate::services::{
    common::Property,
    traits::{ModelMonitoring, Reactive},
};

/// Manages wired (ethernet) network connectivity and device state.
///
/// Provides interface for monitoring ethernet connection status.
/// Unlike WiFi, wired connections are typically automatic and don't
/// require manual connection management or authentication.
#[derive(Clone, Debug)]
pub struct Wired {
    /// The underlying wired device.
    pub device: DeviceWired,

    /// Current wired network connectivity status.
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
            reason: e.to_string(),
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
