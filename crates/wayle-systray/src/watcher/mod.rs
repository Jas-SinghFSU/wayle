#![allow(missing_docs)]
pub(crate) mod discovery;
mod monitoring;

use std::sync::Arc;

use derive_more::Debug;
use tokio::sync::{RwLock, broadcast};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, instrument};
use wayle_traits::ServiceMonitoring;
use zbus::{Connection, fdo, message::Header, object_server::SignalEmitter};

use super::{
    error::Error,
    events::TrayEvent,
    types::{PROTOCOL_VERSION, WATCHER_INTERFACE, WATCHER_OBJECT_PATH},
};

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

pub(crate) async fn register_item(
    service: &str,
    registered_items: &Arc<RwLock<Vec<String>>>,
    event_tx: &broadcast::Sender<TrayEvent>,
    connection: &Connection,
) -> bool {
    let service = service.to_string();

    {
        let mut items = registered_items.write().await;
        if items.contains(&service) {
            return false;
        }
        items.push(service.clone());
    }

    let _ = event_tx.send(TrayEvent::ItemRegistered(service.clone()));

    connection
        .emit_signal(
            None::<()>,
            WATCHER_OBJECT_PATH,
            WATCHER_INTERFACE,
            "StatusNotifierItemRegistered",
            &service,
        )
        .await
        .unwrap_or_else(|err| {
            error!(error = %err, service = %service, "cannot emit item registered signal");
        });

    true
}

#[zbus::interface(name = "org.kde.StatusNotifierWatcher")]
impl StatusNotifierWatcher {
    #[instrument(skip(self, _ctx, header), fields(service = %service))]
    async fn register_status_notifier_item(
        &mut self,
        #[zbus(signal_context)] _ctx: SignalEmitter<'_>,
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

        info!(service = %full_service, "registering StatusNotifierItem");

        register_item(
            &full_service,
            &self.registered_items,
            &self.event_tx,
            &self.zbus_connection,
        )
        .await;

        Ok(())
    }

    #[instrument(skip(self, ctx), fields(service = %service))]
    async fn register_status_notifier_host(
        &mut self,
        #[zbus(signal_context)] ctx: SignalEmitter<'_>,
        service: String,
    ) -> fdo::Result<()> {
        info!(service = %service, "registering StatusNotifierHost");

        let should_signal = {
            let mut hosts = self.registered_hosts.write().await;
            let was_empty = hosts.is_empty();

            if hosts.contains(&service) {
                false
            } else {
                hosts.push(service.clone());
                was_empty
            }
        };

        if should_signal {
            Self::status_notifier_host_registered(&ctx).await?;
        }

        Ok(())
    }

    #[zbus(property)]
    async fn registered_status_notifier_items(&self) -> Vec<String> {
        self.registered_items.read().await.clone()
    }

    #[zbus(property)]
    async fn is_status_notifier_host_registered(&self) -> bool {
        !self.registered_hosts.read().await.is_empty()
    }

    #[zbus(property)]
    fn protocol_version(&self) -> i32 {
        PROTOCOL_VERSION
    }

    #[zbus(signal)]
    async fn status_notifier_item_registered(
        ctx: &SignalEmitter<'_>,
        service: String,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn status_notifier_item_unregistered(
        ctx: &SignalEmitter<'_>,
        service: String,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn status_notifier_host_registered(ctx: &SignalEmitter<'_>) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn status_notifier_host_unregistered(ctx: &SignalEmitter<'_>) -> zbus::Result<()>;
}

impl StatusNotifierWatcher {
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
