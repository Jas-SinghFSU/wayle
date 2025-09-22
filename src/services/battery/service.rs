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
    /// Creates a new battery service for the default DisplayDevice.
    ///
    /// The DisplayDevice is UPower's composite device that represents the overall
    /// battery status, automatically handling multiple batteries if present.
    /// This is the recommended way to monitor battery status for most applications.
    ///
    /// # Errors
    ///
    /// Returns `Error::ServiceInitializationFailed` if service initialization fails.
    pub async fn new() -> Result<Self, Error> {
        Self::builder().build().await
    }

    /// Creates a builder for configuring a BatteryService.
    ///
    /// Use this when you need to monitor a specific battery device
    /// rather than the default aggregated DisplayDevice.
    pub fn builder() -> BatteryServiceBuilder {
        BatteryServiceBuilder::new()
    }
}

/// Builder for configuring a BatteryService.
pub struct BatteryServiceBuilder {
    device_path: Option<OwnedObjectPath>,
}

impl BatteryServiceBuilder {
    /// Creates a new builder with default configuration.
    pub fn new() -> Self {
        Self { device_path: None }
    }

    /// Sets a specific UPower device path.
    ///
    /// If not set, defaults to the DisplayDevice which aggregates all batteries.
    ///
    /// # Arguments
    /// * `path` - D-Bus path to the UPower device (e.g., "/org/freedesktop/UPower/devices/battery_BAT0")
    pub fn device_path(mut self, path: impl Into<OwnedObjectPath>) -> Self {
        self.device_path = Some(path.into());
        self
    }

    /// Builds the BatteryService.
    ///
    /// Uses the DisplayDevice if no specific device path was set.
    /// The DisplayDevice is UPower's composite device that represents the overall
    /// battery status, automatically handling multiple batteries if present.
    ///
    /// # Errors
    ///
    /// Returns `Error::ServiceInitializationFailed` if service initialization fails.
    #[instrument(skip_all)]
    pub async fn build(self) -> Result<BatteryService, Error> {
        let device_path = if let Some(path) = self.device_path {
            path
        } else {
            OwnedObjectPath::try_from("/org/freedesktop/UPower/devices/DisplayDevice")
                .map_err(|e| Error::ServiceInitializationFailed(format!("Invalid path: {e}")))?
        };

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

        Ok(BatteryService { device })
    }
}

impl Default for BatteryServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}
