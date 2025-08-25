use std::sync::Arc;

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::warn;
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::Settings;
use crate::services::network::{
    NetworkError, SettingsProxy, core::settings_connection::ConnectionSettings,
};

pub(super) struct SettingsMonitor;

impl SettingsMonitor {
    pub(super) async fn start(
        zbus_connection: &Connection,
        settings: Arc<Settings>,
        cancellation_token: CancellationToken,
    ) {
        let zbus_connection = zbus_connection.clone();

        tokio::spawn(async move {
            if let Err(e) = Self::monitor(settings, &zbus_connection, cancellation_token).await {
                warn!("Failed to start SettingsMonitor: {e}");
            }
        });
    }

    #[allow(clippy::cognitive_complexity)]
    async fn monitor(
        settings: Arc<Settings>,
        zbus_connection: &Connection,
        cancellation_token: CancellationToken,
    ) -> Result<(), NetworkError> {
        let settings_proxy = SettingsProxy::new(zbus_connection).await?;

        let mut connection_removed = settings_proxy.receive_connection_removed().await;
        let mut connection_added = settings_proxy.receive_new_connection().await;
        let mut hostname_changed = settings_proxy.receive_hostname_changed().await;
        let mut can_modify_changed = settings_proxy.receive_can_modify_changed().await;
        let mut version_id_changed = settings_proxy.receive_version_id_changed().await;

        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    tracing::debug!("SettingsMonitor cancelled");
                    return Ok(());
                }
                Some(event) = async { connection_added.as_mut().ok()?.next().await }, if
                    connection_added.is_ok() => {
                        if let Ok(args) = event.args() {
                            let _ = Self::add_connection(args.connection, &settings, zbus_connection).await;
                        }
                    }
                Some(event) = async { connection_removed.as_mut().ok()?.next().await }, if
                    connection_removed.is_ok() => {
                        if let Ok(args) = event.args() {
                            let _ = Self::remove_connection(args.connection, &settings).await;
                        }
                }
                Some(change) = hostname_changed.next() => {
                    if let Ok(new_hostname) = change.get().await {
                        settings.hostname.set(new_hostname);
                    }
                }
                Some(change) = can_modify_changed.next() => {
                    if let Ok(new_can_modify) = change.get().await {
                        settings.can_modify.set(new_can_modify);
                    }

                }
                Some(change) = version_id_changed.next() => {
                    if let Ok(new_version_id) = change.get().await {
                        settings.version_id.set(new_version_id);
                    }
                }
                else => {
                    warn!("All property streams ended for Settings");
                    break;
                }
            }
        }

        Ok(())
    }

    async fn add_connection(
        connection_path: OwnedObjectPath,
        settings: &Arc<Settings>,
        zbus_connection: &Connection,
    ) -> Result<(), NetworkError> {
        let new_connection =
            ConnectionSettings::get(zbus_connection, connection_path.clone()).await?;

        let mut current_connections = settings.connections.get();

        let found_connection = current_connections
            .iter()
            .find(|connection| connection.object_path.get() == connection_path);

        if found_connection.is_none() {
            current_connections.push((*new_connection).clone());
            settings.connections.set(current_connections);
        }

        Ok(())
    }

    async fn remove_connection(
        connection_path: OwnedObjectPath,
        settings: &Arc<Settings>,
    ) -> Result<(), NetworkError> {
        let mut current_connections = settings.connections.get();
        let found_connection = current_connections
            .iter()
            .find(|connection| connection.object_path.get() == connection_path);

        if found_connection.is_none() {
            return Ok(());
        }

        current_connections.retain(|connection| connection.object_path.get() != connection_path);
        settings.connections.set(current_connections);

        Ok(())
    }
}
