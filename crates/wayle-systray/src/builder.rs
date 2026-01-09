//! Builder for configuring a SystemTrayService.

use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::{info, instrument};
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;
use zbus::Connection;

use crate::{
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
}

impl SystemTrayServiceBuilder {
    /// Creates a new builder with default configuration.
    pub fn new() -> Self {
        Self {
            mode: TrayMode::Auto,
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

    /// Builds the SystemTrayService.
    ///
    /// # Errors
    /// Returns error if service initialization fails.
    #[instrument(skip(self), fields(mode = ?self.mode), err)]
    pub async fn build(self) -> Result<SystemTrayService, Error> {
        let connection = Connection::session().await?;

        let cancellation_token = CancellationToken::new();
        let (event_tx, _) = broadcast::channel(256);

        let is_watcher = match self.mode {
            TrayMode::Watcher => {
                Self::become_watcher(&connection).await?;
                true
            }
            TrayMode::Host => {
                Self::verify_watcher_exists(&connection).await?;
                false
            }
            TrayMode::Auto => Self::try_become_watcher(&connection).await?,
        };

        let service = SystemTrayService {
            cancellation_token,
            event_tx,
            connection,
            is_watcher,
            items: Property::new(Vec::new()),
        };

        if is_watcher {
            let watcher = StatusNotifierWatcher::new(
                service.event_tx.clone(),
                &service.connection,
                &service.cancellation_token,
            )
            .await?;

            service
                .connection
                .object_server()
                .at(WATCHER_OBJECT_PATH, watcher)
                .await?;
        } else {
            let unique_name = service
                .connection
                .unique_name()
                .ok_or_else(|| {
                    Error::ServiceInitializationFailed("Failed to get unique name".to_string())
                })?
                .to_string();

            SystemTrayServiceDiscovery::register_as_host(&service.connection, &unique_name).await?;
        }

        service.start_monitoring().await?;

        let items = SystemTrayServiceDiscovery::discover_items(
            &service.connection,
            &service.cancellation_token,
        )
        .await?;
        service.items.set(items);

        Ok(service)
    }

    #[instrument(skip(connection), err)]
    async fn try_become_watcher(connection: &Connection) -> Result<bool, Error> {
        match connection.request_name(WATCHER_BUS_NAME).await {
            Ok(_) => {
                info!("Operating as StatusNotifierWatcher");
                Ok(true)
            }
            Err(_) => {
                info!("Connecting to existing StatusNotifierWatcher");
                Ok(false)
            }
        }
    }

    #[instrument(skip(connection), err)]
    async fn become_watcher(connection: &Connection) -> Result<(), Error> {
        connection
            .request_name(WATCHER_BUS_NAME)
            .await
            .map_err(|_| Error::WatcherRegistrationFailed("Name already taken".to_string()))?;

        info!("Operating as StatusNotifierWatcher");
        Ok(())
    }

    #[instrument(skip(connection), err)]
    async fn verify_watcher_exists(connection: &Connection) -> Result<(), Error> {
        StatusNotifierWatcherProxy::new(connection)
            .await
            .map_err(|_| {
                Error::ServiceInitializationFailed(
                    "No StatusNotifierWatcher available to connect to".to_string(),
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
