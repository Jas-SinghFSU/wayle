use std::sync::Arc;

use tokio::sync::{RwLock, broadcast};
use tokio_stream::StreamExt;
use tracing::{debug, error, warn};
use wayle_traits::ServiceMonitoring;
use zbus::{Connection, fdo::DBusProxy};

use super::StatusNotifierWatcher;
use crate::{
    error::Error,
    events::TrayEvent,
    types::{WATCHER_INTERFACE, WATCHER_OBJECT_PATH},
};

impl ServiceMonitoring for StatusNotifierWatcher {
    type Error = Error;

    async fn start_monitoring(&self) -> Result<(), Error> {
        monitor_name_owner_changes(self).await
    }
}

async fn monitor_name_owner_changes(watcher: &StatusNotifierWatcher) -> Result<(), Error> {
    let dbus_proxy = match DBusProxy::new(&watcher.zbus_connection).await {
        Ok(proxy) => proxy,
        Err(error) => {
            warn!(error = %error, "cannot create dbus proxy for name monitoring");
            return Ok(());
        }
    };

    let mut name_owner_changed = match dbus_proxy.receive_name_owner_changed().await {
        Ok(stream) => stream,
        Err(error) => {
            warn!(error = %error, "cannot subscribe to NameOwnerChanged");
            return Ok(());
        }
    };

    let cancellation_token = watcher.cancellation_token.clone();
    let connection = watcher.zbus_connection.clone();
    let registered_items = watcher.registered_items.clone();
    let registered_hosts = watcher.registered_hosts.clone();
    let event_tx = watcher.event_tx.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    return;
                }
                Some(signal) = name_owner_changed.next() => {
                    let Ok(args) = signal.args() else {
                        continue;
                    };

                    if args.new_owner.is_some() {
                        continue;
                    }
                    let _ = unregister_item(
                        args.name.as_str(),
                        &registered_items,
                        &event_tx,
                        &connection
                    ).await;

                    let _ = unregister_host(
                        args.name.as_str(),
                        &registered_hosts,
                        &connection
                    ).await;
                }
            }
        }
    });

    Ok(())
}

pub(crate) async fn unregister_item(
    bus_name: &str,
    registered_items: &Arc<RwLock<Vec<String>>>,
    event_tx: &broadcast::Sender<TrayEvent>,
    connection: &Connection,
) -> Result<(), Error> {
    let removed_items = {
        let mut items = registered_items.write().await;
        let prefix = format!("{bus_name}/");

        let mut removed = Vec::new();
        items.retain(|s| {
            if s == bus_name || s.starts_with(&prefix) {
                removed.push(s.clone());
                false
            } else {
                true
            }
        });
        removed
    };

    for item in removed_items {
        let _ = event_tx.send(TrayEvent::ItemUnregistered(item.clone()));

        connection
            .emit_signal(
                None::<()>,
                WATCHER_OBJECT_PATH,
                WATCHER_INTERFACE,
                "StatusNotifierItemUnregistered",
                &item,
            )
            .await
            .unwrap_or_else(|error| {
                error!(error = %error, item = %item, "cannot emit unregistered signal for item");
            });
    }

    Ok(())
}

pub(crate) async fn unregister_host(
    host: &str,
    registered_hosts: &Arc<RwLock<Vec<String>>>,
    connection: &Connection,
) -> Result<(), Error> {
    let is_empty = {
        let mut hosts = registered_hosts.write().await;

        if let Some(index) = hosts.iter().position(|s| s == host) {
            hosts.remove(index);
            debug!("Systray watcher host unregistered: {host}");

            hosts.is_empty()
        } else {
            false
        }
    };

    if is_empty {
        connection
            .emit_signal(
                None::<()>,
                WATCHER_OBJECT_PATH,
                WATCHER_INTERFACE,
                "StatusNotifierHostUnregistered",
                &(),
            )
            .await
            .unwrap_or_else(|error| {
                error!(error = %error, host = %host, "cannot emit unregistered signal for host");
            })
    }

    Ok(())
}
