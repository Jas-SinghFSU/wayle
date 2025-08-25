mod monitoring;

use std::{ops::Deref, sync::Arc};

use monitoring::WiredMonitor;
use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{NetworkError, NetworkStatus, core::device::wired::DeviceWired};
use crate::services::common::Property;

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

impl Deref for Wired {
    type Target = DeviceWired;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl PartialEq for Wired {
    fn eq(&self, other: &Self) -> bool {
        self.device.object_path == other.device.object_path
    }
}

impl Wired {
    /// Get a snapshot of the current wired state (no monitoring).
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::ObjectCreationFailed` if the wired device cannot be created
    pub async fn get(
        connection: &Connection,
        device_path: OwnedObjectPath,
    ) -> Result<Arc<Self>, NetworkError> {
        let device_arc = DeviceWired::get(connection, device_path.clone())
            .await
            .map_err(|e| NetworkError::ObjectCreationFailed {
                object_type: "Wired".to_string(),
                object_path: device_path.clone(),
                reason: e.to_string(),
            })?;
        let device = DeviceWired::clone(&device_arc);

        let wired = Self::from_device(device).await?;
        Ok(Arc::new(wired))
    }

    /// Get a live-updating wired instance (with monitoring).
    ///
    /// Fetches the device, current state and starts monitoring for updates.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::ObjectCreationFailed` if the wired device cannot be created
    /// or if monitoring fails to start
    pub async fn get_live(
        connection: &Connection,
        device_path: OwnedObjectPath,
        cancellation_token: CancellationToken,
    ) -> Result<Arc<Self>, NetworkError> {
        let device_arc =
            DeviceWired::get_live(connection, device_path, cancellation_token.child_token())
                .await?;
        let device = DeviceWired::clone(&device_arc);

        let wired = Self::from_device(device.clone()).await?;
        let wired_arc = Arc::new(wired);

        let _monitoring_handle =
            WiredMonitor::start(connection, &wired_arc, cancellation_token).await?;

        Ok(wired_arc)
    }

    async fn from_device(device: DeviceWired) -> Result<Self, NetworkError> {
        let device_state = &device.state.get();
        let connectivity = NetworkStatus::from_device_state(*device_state);

        Ok(Self {
            device,
            connectivity: Property::new(connectivity),
        })
    }
}
