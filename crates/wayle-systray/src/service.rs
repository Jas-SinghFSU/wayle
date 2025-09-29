use std::sync::Arc;

use derive_more::Debug;
use futures::{Stream, StreamExt};
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::{info, instrument};
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;
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
use crate::proxy::status_notifier_watcher::StatusNotifierWatcherProxy;

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

    /// Whether this service is operating as a StatusNotifierWatcher (registry).
    ///
    /// When `true`, the service acts as the central registry receiving registrations
    /// from tray items. When `false`, the service acts as a host consuming items from
    /// an existing watcher.
    pub is_watcher: bool,

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

    /// A new StatusNotifierItem has been registered, the argument of the signal is the session
    /// bus name of the instance.
    ///
    /// StatusNotifierHost instances should react to this signal by refreshing their
    /// representation of the item list.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn status_notifier_item_registered_signal(
        &self,
    ) -> Result<impl Stream<Item = String>, Error> {
        let proxy = StatusNotifierWatcherProxy::new(&self.connection).await?;
        let stream = proxy.receive_status_notifier_item_registered().await?;

        Ok(stream.filter_map(|signal| async move { signal.args().ok().map(|args| args.service) }))
    }

    /// A StatusNotifierItem instance has disappeared from the bus, the argument of the signal is
    /// the session bus name of the instance.
    ///
    /// StatusNotifierHost instances should react to this signal by refreshing their
    /// representation of the item list.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn status_notifier_item_unregistered_signal(
        &self,
    ) -> Result<impl Stream<Item = String>, Error> {
        let proxy = StatusNotifierWatcherProxy::new(&self.connection).await?;
        let stream = proxy.receive_status_notifier_item_unregistered().await?;

        Ok(stream.filter_map(|signal| async move { signal.args().ok().map(|args| args.service) }))
    }

    /// A new StatusNotifierHost has been registered.
    ///
    /// StatusNotifierItem instances that previously did not register if no hosts were available
    /// may now reconsider to register.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn status_notifier_host_registered_signal(
        &self,
    ) -> Result<impl Stream<Item = ()>, Error> {
        let proxy = StatusNotifierWatcherProxy::new(&self.connection).await?;
        let stream = proxy.receive_status_notifier_host_registered().await?;

        Ok(stream.filter_map(|_signal| async move { Some(()) }))
    }

    /// There are no more StatusNotifierHost instances running.
    ///
    /// StatusNotifierItem instances may choose to skip registration if there are no hosts
    /// available.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn status_notifier_host_unregistered_signal(
        &self,
    ) -> Result<impl Stream<Item = ()>, Error> {
        let proxy = StatusNotifierWatcherProxy::new(&self.connection).await?;
        let stream = proxy.receive_status_notifier_host_unregistered().await?;

        Ok(stream.filter_map(|_signal| async move { Some(()) }))
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

impl Drop for SystemTrayService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
