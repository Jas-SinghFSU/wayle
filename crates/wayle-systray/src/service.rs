use std::sync::Arc;

use derive_more::Debug;
use futures::{Stream, StreamExt};
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::instrument;
use wayle_common::Property;
use zbus::Connection;

use super::{
    core::item::TrayItem,
    error::Error,
    events::TrayEvent,
    proxy::status_notifier_item::StatusNotifierItemProxy,
    types::{Coordinates, ScrollDelta},
};
use crate::{
    builder::SystemTrayServiceBuilder, proxy::status_notifier_watcher::StatusNotifierWatcherProxy,
};

/// System tray service entry point. See [crate-level docs](crate) for usage.
#[derive(Debug)]
pub struct SystemTrayService {
    #[debug(skip)]
    pub(crate) cancellation_token: CancellationToken,
    #[debug(skip)]
    pub(crate) event_tx: broadcast::Sender<TrayEvent>,
    #[debug(skip)]
    pub(crate) connection: Connection,

    /// `true` if acting as the watcher registry, `false` if consuming from existing watcher.
    pub is_watcher: bool,

    /// Currently registered tray items.
    pub items: Property<Vec<Arc<TrayItem>>>,
}

impl SystemTrayService {
    /// Creates a new system tray service with default configuration.
    ///
    /// Automatically detects whether to act as StatusNotifierWatcher
    /// or connect to an existing one.
    ///
    /// For more control over initialization, see [`Self::builder()`].
    ///
    /// # Errors
    /// Returns error if D-Bus connection fails or service initialization fails.
    #[instrument(name = "SystemTrayService::new", err)]
    pub async fn new() -> Result<Arc<Self>, Error> {
        Self::builder().build().await
    }

    /// Returns a builder for configuring the system tray service.
    ///
    /// The builder provides advanced configuration options such as enabling D-Bus
    /// daemon registration for CLI control.
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
    /// StatusNotifierHost instances typically refresh their item list representation in response
    /// to this signal.
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
    /// StatusNotifierHost instances typically refresh their item list representation in response
    /// to this signal.
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
    /// StatusNotifierItem instances that previously skipped registration due to no available hosts
    /// may now proceed with registration.
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
    /// StatusNotifierItem instances can skip registration when no hosts are available.
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

impl Drop for SystemTrayService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
