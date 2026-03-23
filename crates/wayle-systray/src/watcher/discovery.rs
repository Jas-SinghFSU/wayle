use std::{sync::Arc, time::Duration};

use tokio::sync::{RwLock, broadcast};
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, warn};
use zbus::{Connection, fdo::DBusProxy, names::OwnedBusName};

use super::register_item;
use crate::{events::TrayEvent, proxy::status_notifier_item::StatusNotifierItemProxy};

const PROBE_TIMEOUT: Duration = Duration::from_millis(500);

/// Scans the bus for SNI items that didn't re-register after a watcher restart.
pub(crate) fn spawn_orphan_scan(
    connection: Connection,
    registered_items: Arc<RwLock<Vec<String>>>,
    event_tx: broadcast::Sender<TrayEvent>,
    cancellation_token: CancellationToken,
    own_name: String,
) {
    tokio::spawn(scan_bus(
        connection,
        registered_items,
        event_tx,
        cancellation_token,
        own_name,
    ));
}

#[allow(clippy::cognitive_complexity)]
async fn scan_bus(
    connection: Connection,
    registered_items: Arc<RwLock<Vec<String>>>,
    event_tx: broadcast::Sender<TrayEvent>,
    cancellation_token: CancellationToken,
    own_name: String,
) {
    let Some(candidates) = list_candidate_names(&connection, &own_name).await else {
        return;
    };

    debug!(
        count = candidates.len(),
        "scanning bus for orphaned SNI items"
    );

    let mut found = 0u32;

    for bus_name in &candidates {
        if cancellation_token.is_cancelled() {
            return;
        }

        let name_str = bus_name.as_str();

        if is_registered(&registered_items, name_str).await {
            continue;
        }

        if !probe_sni(&connection, name_str).await {
            continue;
        }

        if register_item(name_str, &registered_items, &event_tx, &connection).await {
            info!(service = %name_str, "recovered orphaned SNI item");
            found += 1;
        }
    }

    if found > 0 {
        info!(count = found, "recovered orphaned SNI items");
    }
}

async fn list_candidate_names(
    connection: &Connection,
    own_name: &str,
) -> Option<Vec<OwnedBusName>> {
    let dbus_proxy = match DBusProxy::new(connection).await {
        Ok(proxy) => proxy,
        Err(error) => {
            warn!(error = %error, "cannot create DBus proxy for orphan scan");
            return None;
        }
    };

    let bus_names = match dbus_proxy.list_names().await {
        Ok(names) => names,
        Err(error) => {
            warn!(error = %error, "cannot list bus names for orphan scan");
            return None;
        }
    };

    let candidates = bus_names
        .into_iter()
        .filter(|name| {
            let name = name.as_str();
            name.starts_with(':') && name != own_name
        })
        .collect();

    Some(candidates)
}

async fn is_registered(items: &Arc<RwLock<Vec<String>>>, bus_name: &str) -> bool {
    let items = items.read().await;
    let prefix = format!("{bus_name}/");

    items
        .iter()
        .any(|registered| registered == bus_name || registered.starts_with(&prefix))
}

async fn probe_sni(connection: &Connection, bus_name: &str) -> bool {
    let probe = async {
        let proxy = StatusNotifierItemProxy::builder(connection)
            .destination(bus_name)?
            .build()
            .await?;
        proxy.id().await
    };

    matches!(tokio::time::timeout(PROBE_TIMEOUT, probe).await, Ok(Ok(_)))
}
