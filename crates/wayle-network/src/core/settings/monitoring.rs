use std::sync::{Arc, Weak};

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};
use wayle_common::remove_and_cancel;
use wayle_traits::{ModelMonitoring, Reactive};
use zbus::zvariant::OwnedObjectPath;

use super::Settings;
use crate::{
    core::settings_connection::{ConnectionSettings, ConnectionSettingsParams},
    error::Error,
    proxy::settings::SettingsProxy,
};

impl ModelMonitoring for Settings {
    type Error = Error;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(Error::MissingCancellationToken);
        };

        let settings_proxy = SettingsProxy::new(&self.zbus_connection).await?;
        let cancel_token = cancellation_token.clone();
        let weak_self = Arc::downgrade(&self);

        tokio::spawn(async move {
            if let Err(e) = monitor(weak_self, settings_proxy, cancel_token).await {
                warn!(error = %e, "cannot start settings monitor");
            }
        });

        Ok(())
    }
}

#[allow(clippy::cognitive_complexity)]
async fn monitor(
    weak_settings: Weak<Settings>,
    settings_proxy: SettingsProxy<'_>,
    cancellation_token: CancellationToken,
) -> Result<(), Error> {
    let mut connection_removed = settings_proxy.receive_connection_removed().await;
    let mut connection_added = settings_proxy.receive_new_connection().await;
    let mut hostname_changed = settings_proxy.receive_hostname_changed().await;
    let mut can_modify_changed = settings_proxy.receive_can_modify_changed().await;
    let mut version_id_changed = settings_proxy.receive_version_id_changed().await;

    loop {
        let Some(settings) = weak_settings.upgrade() else {
            return Ok(());
        };

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
) -> Result<(), Error> {
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
) -> Result<(), Error> {
    remove_and_cancel!(settings.connections.clone(), connection_path);
    Ok(())
}
