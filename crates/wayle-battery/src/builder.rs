//! Builder for configuring a BatteryService.

use tokio_util::sync::CancellationToken;
use tracing::instrument;
use wayle_traits::Reactive;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::{
    core::device::{Device, types::LiveDeviceParams},
    error::Error,
    service::BatteryService,
};

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
    /// Returns `Error::InvalidObjectPath` if the device path is invalid, or
    /// `Error::Dbus` if D-Bus connection fails.
    #[instrument(skip_all)]
    pub async fn build(self) -> Result<BatteryService, Error> {
        let device_path = if let Some(path) = self.device_path {
            path
        } else {
            OwnedObjectPath::try_from("/org/freedesktop/UPower/devices/DisplayDevice")?
        };

        let connection = Connection::system().await?;

        let cancellation_token = CancellationToken::new();

        let device = Device::get_live(LiveDeviceParams {
            connection: &connection,
            device_path: &device_path,
            cancellation_token: &cancellation_token,
        })
        .await?;

        Ok(BatteryService {
            device,
            cancellation_token,
        })
    }
}

impl Default for BatteryServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}
