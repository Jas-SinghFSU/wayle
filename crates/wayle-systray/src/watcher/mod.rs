#![allow(missing_docs)]
mod monitoring;

use std::sync::Arc;

use derive_more::Debug;
use tokio::sync::{RwLock, broadcast};
use tokio_util::sync::CancellationToken;
use tracing::{info, instrument};
use wayle_traits::ServiceMonitoring;
use zbus::{Connection, fdo, message::Header, object_server::SignalEmitter};

use super::{error::Error, events::TrayEvent, types::PROTOCOL_VERSION};

/// StatusNotifierWatcher D-Bus interface implementation.
///
/// Acts as the central registry for StatusNotifierItems and Hosts.
#[derive(Debug)]
pub(crate) struct StatusNotifierWatcher {
    #[debug(skip)]
    pub zbus_connection: Connection,
    #[debug(skip)]
    pub event_tx: broadcast::Sender<TrayEvent>,
    #[debug(skip)]
    pub cancellation_token: CancellationToken,

    pub registered_items: Arc<RwLock<Vec<String>>>,
    pub registered_hosts: Arc<RwLock<Vec<String>>>,
}

#[zbus::interface(name = "org.kde.StatusNotifierWatcher")]
impl StatusNotifierWatcher {
    /// Register a StatusNotifierItem into the watcher.
    ///
    /// The service string can be either a bus name (searched at /StatusNotifierItem)
    /// or a full object path. When a full object path is provided, the sender's bus
    /// name is prepended to form the complete identifier.
    #[instrument(skip(self, ctx, header), fields(service = %service))]
    async fn register_status_notifier_item(
        &mut self,
        #[zbus(signal_context)] ctx: SignalEmitter<'_>,
        #[zbus(header)] header: Header<'_>,
        service: String,
    ) -> fdo::Result<()> {
        let full_service = if service.starts_with('/') {
            let sender = header
                .sender()
                .ok_or_else(|| fdo::Error::Failed("No sender in D-Bus message header".into()))?;
            format!("{sender}{service}")
        } else {
            service
        };

        info!("Registering StatusNotifierItem: {}", full_service);

        let mut items = self.registered_items.write().await;
        if !items.contains(&full_service) {
            items.push(full_service.clone());
            drop(items);

            let _ = self
                .event_tx
                .send(TrayEvent::ItemRegistered(full_service.clone()));
            Self::status_notifier_item_registered(&ctx, full_service).await?;
        }
        Ok(())
    }

    /// Register a StatusNotifierHost into the watcher.
    ///
    /// Every host that intends to display StatusNotifierItems should register.
    #[instrument(skip(self, ctx), fields(service = %service))]
    async fn register_status_notifier_host(
        &mut self,
        #[zbus(signal_context)] ctx: SignalEmitter<'_>,
        service: String,
    ) -> fdo::Result<()> {
        info!("Registering StatusNotifierHost: {}", service);

        let mut hosts = self.registered_hosts.write().await;
        let was_empty = hosts.is_empty();

        if !hosts.contains(&service) {
            hosts.push(service.clone());
            drop(hosts);

            if was_empty {
                Self::status_notifier_host_registered(&ctx).await?;
            }
        }
        Ok(())
    }

    /// List of all registered StatusNotifierItems.
    #[zbus(property)]
    async fn registered_status_notifier_items(&self) -> Vec<String> {
        self.registered_items.read().await.clone()
    }

    /// Whether at least one StatusNotifierHost has been registered.
    #[zbus(property)]
    async fn is_status_notifier_host_registered(&self) -> bool {
        !self.registered_hosts.read().await.is_empty()
    }

    /// Protocol version (always 0 for this specification).
    #[zbus(property)]
    fn protocol_version(&self) -> i32 {
        PROTOCOL_VERSION
    }

    /// Signal: A new StatusNotifierItem has been registered.
    #[zbus(signal)]
    async fn status_notifier_item_registered(
        ctx: &SignalEmitter<'_>,
        service: String,
    ) -> zbus::Result<()>;

    /// Signal: A StatusNotifierItem has been unregistered.
    #[zbus(signal)]
    async fn status_notifier_item_unregistered(
        ctx: &SignalEmitter<'_>,
        service: String,
    ) -> zbus::Result<()>;

    /// Signal: A new StatusNotifierHost has been registered.
    #[zbus(signal)]
    async fn status_notifier_host_registered(ctx: &SignalEmitter<'_>) -> zbus::Result<()>;

    /// Signal: There are no more StatusNotifierHost instances.
    #[zbus(signal)]
    async fn status_notifier_host_unregistered(ctx: &SignalEmitter<'_>) -> zbus::Result<()>;
}

impl StatusNotifierWatcher {
    /// Creates a new StatusNotifierWatcher with an initial host pre-registered.
    pub(crate) async fn with_initial_host(
        event_tx: broadcast::Sender<TrayEvent>,
        connection: &Connection,
        cancellation_token: &CancellationToken,
        initial_host: String,
    ) -> Result<Self, Error> {
        let registered_items = Arc::new(RwLock::new(Vec::new()));
        let registered_hosts = Arc::new(RwLock::new(vec![initial_host]));

        let watcher = Self {
            zbus_connection: connection.clone(),
            event_tx,
            cancellation_token: cancellation_token.clone(),
            registered_items,
            registered_hosts,
        };

        watcher.start_monitoring().await?;

        Ok(watcher)
    }
}
