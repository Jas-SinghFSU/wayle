use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use tracing::instrument;
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{
    core::device::{Device, types::LiveDeviceParams},
    error::Error,
};
use crate::services::traits::Reactive;

/// Battery service for monitoring power devices via UPower.
///
/// Provides a high-level interface to UPower's battery monitoring,
/// allowing access to battery state, capacity, charge status, and reactive
/// monitoring of changes through the D-Bus interface.
#[derive(Debug)]
pub struct BatteryService {
    /// The UPower battery device proxy for power metrics and charging state.
    pub device: Arc<Device>,
}

impl BatteryService {
    /// Creates a new battery service instance for the display device.
    ///
    /// The display device is UPower's composite device that represents the overall
    /// battery status, automatically handling multiple batteries if present.
    /// This is the recommended way to monitor battery status for most applications.
    ///
    /// # Errors
    ///
    /// Returns `Error::ServiceInitializationFailed` if service initialization fails.
    #[instrument]
    pub async fn new() -> Result<Self, Error> {
        let device_path =
            OwnedObjectPath::try_from("/org/freedesktop/UPower/devices/DisplayDevice")
                .map_err(|e| Error::ServiceInitializationFailed(format!("Invalid path: {e}")))?;

        Self::with_device(device_path).await
    }

    /// Creates a battery service for a specific UPower device.
    ///
    /// Use this when you need to monitor a specific battery or power device
    /// rather than the aggregated display device.
    ///
    /// # Arguments
    /// * `device_path` - D-Bus path to the UPower device (e.g., "/org/freedesktop/UPower/devices/battery_BAT0")
    ///
    /// # Errors
    ///
    /// Returns `Error::ServiceInitializationFailed` if service initialization fails.
    pub async fn with_device(device_path: OwnedObjectPath) -> Result<Self, Error> {
        let connection = Connection::system().await.map_err(|err| {
            Error::ServiceInitializationFailed(format!("D-Bus connection failed: {err}"))
        })?;

        let cancellation_token = CancellationToken::new();

        let device = Device::get_live(LiveDeviceParams {
            connection: &connection,
            device_path: &device_path,
            cancellation_token: &cancellation_token,
        })
        .await?;

        Ok(Self { device })
    }
}
