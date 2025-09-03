use std::sync::Arc;

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};
use zbus::zvariant::OwnedObjectPath;

use super::Settings;
use crate::{
    remove_and_cancel,
    services::{
        network::{
            NetworkError, SettingsProxy,
            core::settings_connection::{ConnectionSettings, ConnectionSettingsParams},
        },
        traits::{ModelMonitoring, Reactive},
    },
};

impl ModelMonitoring for Settings {
    type Error = NetworkError;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(NetworkError::OperationFailed {
                operation: "start_monitoring",
                reason: String::from("A cancellation_token was not found."),
            });
        };

        let cancel_token = cancellation_token.clone();

        tokio::spawn(async move {
            if let Err(e) = monitor(self, cancel_token).await {
                warn!("Failed to start SettingsMonitor: {e}");
            }
        });

        Ok(())
    }
}

#[allow(clippy::cognitive_complexity)]
async fn monitor(
    settings: Arc<Settings>,
    cancellation_token: CancellationToken,
) -> Result<(), NetworkError> {
    let settings_proxy = SettingsProxy::new(&settings.zbus_connection).await?;

    let mut connection_removed = settings_proxy.receive_connection_removed().await;
    let mut connection_added = settings_proxy.receive_new_connection().await;
    let mut hostname_changed = settings_proxy.receive_hostname_changed().await;
    let mut can_modify_changed = settings_proxy.receive_can_modify_changed().await;
    let mut version_id_changed = settings_proxy.receive_version_id_changed().await;

    loop {
        tokio::select! {
            _ = cancellation_token.cancelled() => {
                debug!("SettingsMonitor cancelled");
                return Ok(());
            }
            Some(event) = async { connection_added.as_mut().ok()?.next().await }, if
                connection_added.is_ok() => {
                    if let Ok(args) = event.args() {
                        let _ = add_connection(args.connection, &settings).await;
                    }
                }
            Some(event) = async { connection_removed.as_mut().ok()?.next().await }, if
                connection_removed.is_ok() => {
                    if let Ok(args) = event.args() {
                        let _ = remove_connection(args.connection, &settings).await;
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
) -> Result<(), NetworkError> {
    let new_connection = ConnectionSettings::get(ConnectionSettingsParams {
        connection: &settings.zbus_connection,
        path: connection_path.clone(),
    })
    .await?;

    let mut current_connections = settings.connections.get();

    let found_connection = current_connections
        .iter()
        .find(|connection| connection.object_path == connection_path);

    if found_connection.is_none() {
        current_connections.push(new_connection);
        settings.connections.set(current_connections);
    }

    Ok(())
}

async fn remove_connection(
    connection_path: OwnedObjectPath,
    settings: &Arc<Settings>,
) -> Result<(), NetworkError> {
    remove_and_cancel!(settings.connections.clone(), connection_path);
    Ok(())
}
