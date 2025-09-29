use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use tracing::{debug, instrument, warn};
use wayle_traits::Reactive;
use zbus::Connection;

use super::{
    core::item::{LiveTrayItemParams, TrayItem},
    error::Error,
    proxy::status_notifier_watcher::StatusNotifierWatcherProxy,
};

/// Handles discovery of tray items and host registration.
pub(crate) struct SystemTrayServiceDiscovery;

impl SystemTrayServiceDiscovery {
    /// Discovers and creates all existing tray items from the watcher.
    ///
    /// Queries the StatusNotifierWatcher for all registered items and creates
    /// TrayItem objects with their properties fetched and monitoring enabled.
    ///
    /// # Errors
    /// Returns error if watcher connection fails or item creation fails.
    #[instrument(skip(connection, cancellation_token), err)]
    pub async fn discover_items(
        connection: &Connection,
        cancellation_token: &CancellationToken,
    ) -> Result<Vec<Arc<TrayItem>>, Error> {
        let watcher = StatusNotifierWatcherProxy::new(connection).await?;
        let bus_names = watcher.registered_status_notifier_items().await?;

        debug!("Discovered {} existing tray items", bus_names.len());

        let mut items = Vec::with_capacity(bus_names.len());

        for bus_name in bus_names {
            let params = LiveTrayItemParams {
                connection,
                service: bus_name.clone(),
                cancellation_token,
            };

            match TrayItem::get_live(params).await {
                Ok(item) => items.push(item),
                Err(e) => warn!("Failed to load tray item {}: {}", bus_name, e),
            }
        }

        debug!("Successfully loaded {} tray items", items.len());
        Ok(items)
    }

    #[instrument(skip(connection), fields(host_name = %host_name), err)]
    pub async fn register_as_host(connection: &Connection, host_name: &str) -> Result<(), Error> {
        let watcher = StatusNotifierWatcherProxy::new(connection).await?;
        watcher.register_status_notifier_host(host_name).await?;

        debug!("Registered as StatusNotifierHost");
        Ok(())
    }
}
