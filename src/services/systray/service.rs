use std::sync::Arc;

use derive_more::Debug;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::{info, instrument};
use zbus::Connection;

use super::{
    core::item::TrayItem,
    discovery::SystemTrayServiceDiscovery,
    error::Error,
    events::TrayEvent,
    proxy::status_notifier_item::StatusNotifierItemProxy,
    types::{Coordinates, ScrollDelta, TrayMode, WATCHER_BUS_NAME, WATCHER_OBJECT_PATH},
    watcher::StatusNotifierWatcher,
};
use crate::services::{
    common::Property, systray::proxy::status_notifier_watcher::StatusNotifierWatcherProxy,
    traits::ServiceMonitoring,
};

/// System tray service implementing the StatusNotifier protocol.
///
/// Provides discovery and management of system tray items via D-Bus.
/// Automatically detects whether to act as watcher or connect to existing one.
#[derive(Debug)]
pub struct SystemTrayService {
    #[debug(skip)]
    pub(crate) cancellation_token: CancellationToken,
    #[debug(skip)]
    pub(crate) event_tx: broadcast::Sender<TrayEvent>,
    #[debug(skip)]
    pub(crate) connection: Connection,
    pub(crate) is_watcher: bool,

    /// All discovered tray items.
    pub items: Property<Vec<Arc<TrayItem>>>,
}

impl SystemTrayService {
    /// Creates a new system tray service.
    ///
    /// Automatically detects whether to act as StatusNotifierWatcher
    /// or connect to an existing one.
    ///
    /// # Errors
    /// Returns error if D-Bus connection fails or service initialization fails.
    #[instrument(name = "SystemTrayService::new", err)]
    pub async fn new() -> Result<Self, Error> {
        Self::builder().build().await
    }

    /// Creates a builder for configuring a SystemTrayService.
    pub fn builder() -> SystemTrayServiceBuilder {
        SystemTrayServiceBuilder::new()
    }

    /// Activates a tray item (left click).
    ///
    /// # Errors
    /// Returns error if the item doesn't exist or activation fails.
    #[instrument(skip(self), fields(service = %service, x = %coords.x, y = %coords.y), err)]
    pub async fn activate(&self, service: &str, coords: Coordinates) -> Result<(), Error> {
        let proxy = StatusNotifierItemProxy::builder(&self.connection)
            .destination(service)?
            .build()
            .await?;

        proxy.activate(coords.x, coords.y).await?;
        Ok(())
    }

    /// Shows context menu for a tray item (right click).
    ///
    /// # Errors
    /// Returns error if the item doesn't exist or menu activation fails.
    #[instrument(skip(self), fields(service = %service, x = %coords.x, y = %coords.y), err)]
    pub async fn context_menu(&self, service: &str, coords: Coordinates) -> Result<(), Error> {
        let proxy = StatusNotifierItemProxy::builder(&self.connection)
            .destination(service)?
            .build()
            .await?;

        proxy.context_menu(coords.x, coords.y).await?;
        Ok(())
    }

    /// Performs secondary activation (middle click).
    ///
    /// # Errors
    /// Returns error if the item doesn't exist or activation fails.
    #[instrument(skip(self), fields(service = %service, x = %coords.x, y = %coords.y), err)]
    pub async fn secondary_activate(
        &self,
        service: &str,
        coords: Coordinates,
    ) -> Result<(), Error> {
        let proxy = StatusNotifierItemProxy::builder(&self.connection)
            .destination(service)?
            .build()
            .await?;

        proxy.secondary_activate(coords.x, coords.y).await?;
        Ok(())
    }

    /// Scrolls on a tray item.
    ///
    /// # Errors
    /// Returns error if the item doesn't exist or scroll fails.
    #[instrument(
        skip(self),
        fields(service = %service, delta = %scroll.delta, orientation = %scroll.orientation),
        err
    )]
    pub async fn scroll(&self, service: &str, scroll: ScrollDelta) -> Result<(), Error> {
        let proxy = StatusNotifierItemProxy::builder(&self.connection)
            .destination(service)?
            .build()
            .await?;

        proxy
            .scroll(scroll.delta, &scroll.orientation.to_string())
            .await?;
        Ok(())
    }

    /// Returns whether this service is acting as the StatusNotifierWatcher.
    pub fn is_watcher(&self) -> bool {
        self.is_watcher
    }

    /// Shuts down the service gracefully.
    pub async fn shutdown(&self) {
        self.cancellation_token.cancel();
    }
}

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

        let items = if is_watcher {
            let watcher =
                StatusNotifierWatcher::new(event_tx.clone(), &connection, &cancellation_token)
                    .await?;

            connection
                .object_server()
                .at(WATCHER_OBJECT_PATH, watcher)
                .await?;

            Vec::new()
        } else {
            let unique_name = connection
                .unique_name()
                .ok_or_else(|| {
                    Error::ServiceInitializationFailed("Failed to get unique name".to_string())
                })?
                .to_string();

            SystemTrayServiceDiscovery::register_as_host(&connection, &unique_name).await?;
            SystemTrayServiceDiscovery::discover_items(&connection, &cancellation_token).await?
        };

        let service = SystemTrayService {
            cancellation_token,
            event_tx,
            connection,
            is_watcher,
            items: Property::new(items),
        };

        service.start_monitoring().await?;
        Ok(service)
    }

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

    async fn become_watcher(connection: &Connection) -> Result<(), Error> {
        connection
            .request_name(WATCHER_BUS_NAME)
            .await
            .map_err(|_| Error::WatcherRegistrationFailed("Name already taken".to_string()))?;

        info!("Operating as StatusNotifierWatcher");
        Ok(())
    }

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
