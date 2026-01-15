//! Builder for configuring a PowerProfilesService.

use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use tracing::info;
use wayle_traits::Reactive;
use zbus::Connection;

use crate::{
    core::{PowerProfiles, types::LivePowerProfilesParams},
    dbus::{PowerProfilesDaemon, SERVICE_NAME, SERVICE_PATH},
    error::Error,
    service::PowerProfilesService,
};

/// Builder for configuring and creating a PowerProfilesService instance.
///
/// Allows optional D-Bus daemon registration for external control.
#[derive(Default)]
pub struct PowerProfilesServiceBuilder {
    register_daemon: bool,
}

impl PowerProfilesServiceBuilder {
    /// Creates a new PowerProfilesServiceBuilder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables D-Bus daemon registration for external control.
    ///
    /// When enabled, the service will register itself on the session bus
    /// at `com.wayle.PowerProfiles1`, allowing CLI tools and other applications
    /// to control power profiles.
    pub fn with_daemon(mut self) -> Self {
        self.register_daemon = true;
        self
    }

    /// Builds and initializes the PowerProfilesService.
    ///
    /// This will establish a system D-Bus connection and start monitoring
    /// for power profile changes. If `with_daemon()` was called, the service
    /// will also register on the session bus for external control.
    ///
    /// # Errors
    /// Returns error if D-Bus connection fails or monitoring cannot be started.
    pub async fn build(self) -> Result<Arc<PowerProfilesService>, Error> {
        let system_connection = Connection::system().await.map_err(|err| {
            Error::ServiceInitializationFailed(format!("System D-Bus connection failed: {err}"))
        })?;

        let cancellation_token = CancellationToken::new();

        let power_profiles = PowerProfiles::get_live(LivePowerProfilesParams {
            connection: &system_connection,
            cancellation_token: &cancellation_token,
        })
        .await?;

        let session_connection = if self.register_daemon {
            let conn = Connection::session().await.map_err(|err| {
                Error::ServiceInitializationFailed(format!(
                    "Session D-Bus connection failed: {err}"
                ))
            })?;
            Some(conn)
        } else {
            None
        };

        let service = Arc::new(PowerProfilesService {
            power_profiles,
            cancellation_token,
            _connection: session_connection.clone(),
        });

        if let Some(connection) = session_connection {
            let daemon = PowerProfilesDaemon {
                service: Arc::clone(&service),
            };

            connection
                .object_server()
                .at(SERVICE_PATH, daemon)
                .await
                .map_err(|err| {
                    Error::ServiceInitializationFailed(format!(
                        "cannot register D-Bus object at '{SERVICE_PATH}': {err}"
                    ))
                })?;

            connection.request_name(SERVICE_NAME).await.map_err(|err| {
                Error::ServiceInitializationFailed(format!(
                    "cannot acquire D-Bus name '{SERVICE_NAME}': {err}"
                ))
            })?;

            info!("Power profiles service registered at {SERVICE_NAME}");
        }

        Ok(service)
    }
}
