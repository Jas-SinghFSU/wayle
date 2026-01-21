use std::sync::Arc;

use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::{info, instrument};
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;
use zbus::Connection;

use crate::{
    dbus::{SERVICE_NAME, SERVICE_PATH, SystemTrayDaemon},
    discovery::SystemTrayServiceDiscovery,
    error::Error,
    proxy::status_notifier_watcher::StatusNotifierWatcherProxy,
    service::SystemTrayService,
    types::{TrayMode, WATCHER_BUS_NAME, WATCHER_OBJECT_PATH},
    watcher::StatusNotifierWatcher,
};

/// Builder for configuring a SystemTrayService.
pub struct SystemTrayServiceBuilder {
    mode: TrayMode,
    register_daemon: bool,
}

impl SystemTrayServiceBuilder {
    /// Creates a new builder with default configuration.
    pub fn new() -> Self {
        Self {
            mode: TrayMode::Auto,
            register_daemon: false,
        }
    }

    /// Sets the operating mode for the service.
    ///
    /// - `TrayMode::Watcher` - Act as the StatusNotifierWatcher registry
    /// - `TrayMode::Host` - Act as a StatusNotifierHost consumer
    /// - `TrayMode::Auto` - Auto-detect based on name availability (default)
    pub fn mode(mut self, mode: TrayMode) -> Self {
        self.mode = mode;
        self
    }

    /// Enables the Wayle D-Bus daemon for CLI control.
    ///
    /// When enabled, the service registers at `com.wayle.SystemTray1`,
    /// allowing CLI tools to list and activate tray items.
    pub fn with_daemon(mut self) -> Self {
        self.register_daemon = true;
        self
    }

    /// Builds the SystemTrayService.
    ///
    /// # Errors
    /// Returns error if service initialization fails.
    #[instrument(skip(self), fields(mode = ?self.mode), err)]
    pub async fn build(self) -> Result<Arc<SystemTrayService>, Error> {
        let connection = Connection::session().await?;

        let cancellation_token = CancellationToken::new();
        let (event_tx, _) = broadcast::channel(256);

        let unique_name = connection
            .unique_name()
            .ok_or_else(|| {
                Error::ServiceInitialization(
                    "cannot get D-Bus unique name - connection may not be established".to_string(),
                )
            })?
            .to_string();

        let is_watcher = match self.mode {
            TrayMode::Host => {
                Self::verify_watcher_exists(&connection).await?;
                false
            }
            TrayMode::Watcher | TrayMode::Auto => {
                Self::setup_as_watcher(
                    &connection,
                    event_tx.clone(),
                    &cancellation_token,
                    &unique_name,
                    self.mode == TrayMode::Watcher,
                )
                .await?
            }
        };

        let service = Arc::new(SystemTrayService {
            cancellation_token,
            event_tx,
            connection,
            is_watcher,
            items: Property::new(Vec::new()),
        });

        if !is_watcher {
            SystemTrayServiceDiscovery::register_as_host(&service.connection, &unique_name).await?;
        }

        service.start_monitoring().await?;

        if !is_watcher {
            let items = SystemTrayServiceDiscovery::discover_items(
                &service.connection,
                &service.cancellation_token,
            )
            .await?;
            service.items.set(items);
        }

        if self.register_daemon {
            let daemon = SystemTrayDaemon {
                service: Arc::clone(&service),
            };

            service
                .connection
                .object_server()
                .at(SERVICE_PATH, daemon)
                .await
                .map_err(|err| {
                    Error::ServiceInitialization(format!(
                        "cannot register D-Bus object at '{SERVICE_PATH}': {err}"
                    ))
                })?;

            service
                .connection
                .request_name(SERVICE_NAME)
                .await
                .map_err(|err| {
                    Error::ServiceInitialization(format!(
                        "cannot acquire D-Bus name '{SERVICE_NAME}': {err}"
                    ))
                })?;

            info!("System tray service registered at {SERVICE_NAME}");
        }

        Ok(service)
    }

    /// Sets up as watcher
    ///
    /// Returns `true` if successfully became watcher, `false` if fell back to host.
    #[instrument(skip(connection, event_tx, cancellation_token), err)]
    async fn setup_as_watcher(
        connection: &Connection,
        event_tx: broadcast::Sender<crate::events::TrayEvent>,
        cancellation_token: &CancellationToken,
        unique_name: &str,
        force: bool,
    ) -> Result<bool, Error> {
        let watcher = StatusNotifierWatcher::with_initial_host(
            event_tx,
            connection,
            cancellation_token,
            unique_name.to_string(),
        )
        .await?;

        connection
            .object_server()
            .at(WATCHER_OBJECT_PATH, watcher)
            .await?;

        match connection.request_name(WATCHER_BUS_NAME).await {
            Ok(_) => {
                info!("Operating as StatusNotifierWatcher");
                Ok(true)
            }
            Err(err) => {
                if force {
                    return Err(Error::WatcherRegistration(format!(
                        "D-Bus name '{WATCHER_BUS_NAME}' already taken: {err}"
                    )));
                }

                info!("Connecting to existing StatusNotifierWatcher");
                let _ = connection
                    .object_server()
                    .remove::<StatusNotifierWatcher, _>(WATCHER_OBJECT_PATH)
                    .await;
                Ok(false)
            }
        }
    }

    #[instrument(skip(connection), err)]
    async fn verify_watcher_exists(connection: &Connection) -> Result<(), Error> {
        StatusNotifierWatcherProxy::new(connection)
            .await
            .map_err(|_| {
                Error::ServiceInitialization(
                    "no StatusNotifierWatcher available to connect to".to_string(),
                )
            })?;

        info!("Connecting to existing StatusNotifierWatcher as host");
        Ok(())
    }
}

impl Default for SystemTrayServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}
