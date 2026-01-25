use std::sync::Arc;

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};
use wayle_common::Property;
use wayle_traits::{Reactive, ServiceMonitoring};
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{
    discovery::NetworkServiceDiscovery,
    error::Error,
    proxy::manager::NetworkManagerProxy,
    service::NetworkService,
    types::connectivity::ConnectionType,
    wifi::{LiveWifiParams, Wifi},
    wired::{LiveWiredParams, Wired},
};

impl ServiceMonitoring for NetworkService {
    type Error = Error;

    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        spawn_primary_monitoring(
            self.zbus_connection.clone(),
            self.wifi.clone(),
            self.wired.clone(),
            self.primary.clone(),
            self.cancellation_token.child_token(),
        )
        .await?;

        spawn_device_monitoring(
            self.zbus_connection.clone(),
            self.wifi.clone(),
            self.wired.clone(),
            self.cancellation_token.child_token(),
        )
        .await
    }
}

async fn spawn_primary_monitoring(
    connection: Connection,
    wifi: Property<Option<Arc<Wifi>>>,
    wired: Property<Option<Arc<Wired>>>,
    primary: Property<ConnectionType>,
    cancellation_token: CancellationToken,
) -> Result<(), Error> {
    let nm_proxy = NetworkManagerProxy::new(&connection)
        .await
        .map_err(Error::DbusError)?;

    let initial_primary = nm_proxy.primary_connection().await?;
    update_primary_connection(&initial_primary, &wifi, &wired, &primary);

    let mut primary_changed = nm_proxy.receive_primary_connection_changed().await;

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("NetworkMonitoring primary monitoring cancelled");
                    return;
                }
                Some(change) = primary_changed.next() => {
                    if let Ok(new_primary) = change.get().await {
                        debug!(path = %new_primary, "Primary connection changed");
                        update_primary_connection(&new_primary, &wifi, &wired, &primary);
                    }
                }
            }
        }
    });

    Ok(())
}

async fn spawn_device_monitoring(
    connection: Connection,
    wifi: Property<Option<Arc<Wifi>>>,
    wired: Property<Option<Arc<Wired>>>,
    cancellation_token: CancellationToken,
) -> Result<(), Error> {
    let nm_proxy = NetworkManagerProxy::new(&connection)
        .await
        .map_err(Error::DbusError)?;

    let mut device_added = nm_proxy.receive_device_added().await?;
    let mut device_removed = nm_proxy.receive_device_removed().await?;

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("NetworkMonitoring device monitoring cancelled");
                    return;
                }
                Some(signal) = device_added.next() => {
                    let Ok(args) = signal.args() else { continue };
                    debug!(path = %args.device_path, "Network device added");

                    try_initialize_wifi(&connection, &wifi, &cancellation_token).await;
                    try_initialize_wired(&connection, &wired, &cancellation_token).await;
                }
                Some(signal) = device_removed.next() => {
                    let Ok(args) = signal.args() else { continue };
                    debug!(path = %args.device_path, "Network device removed");

                    handle_wifi_removed(&args.device_path, &wifi);
                    handle_wired_removed(&args.device_path, &wired);
                }
            }
        }
    });

    Ok(())
}

async fn try_initialize_wifi(
    connection: &Connection,
    wifi: &Property<Option<Arc<Wifi>>>,
    cancellation_token: &CancellationToken,
) {
    if wifi.get().is_some() {
        return;
    }

    let Some(path) = NetworkServiceDiscovery::wifi_device_path(connection)
        .await
        .ok()
        .flatten()
    else {
        return;
    };

    match Wifi::get_live(LiveWifiParams {
        connection,
        device_path: path.clone(),
        cancellation_token,
    })
    .await
    {
        Ok(new_wifi) => {
            debug!(path = %path, "WiFi device initialized");
            wifi.set(Some(new_wifi));
        }
        Err(err) => {
            warn!(error = %err, path = %path, "Failed to initialize WiFi device");
        }
    }
}

async fn try_initialize_wired(
    connection: &Connection,
    wired: &Property<Option<Arc<Wired>>>,
    cancellation_token: &CancellationToken,
) {
    if wired.get().is_some() {
        return;
    }

    let Some(path) = NetworkServiceDiscovery::wired_device_path(connection)
        .await
        .ok()
        .flatten()
    else {
        return;
    };

    match Wired::get_live(LiveWiredParams {
        connection,
        device_path: path.clone(),
        cancellation_token,
    })
    .await
    {
        Ok(new_wired) => {
            debug!(path = %path, "Wired device initialized");
            wired.set(Some(new_wired));
        }
        Err(err) => {
            warn!(error = %err, path = %path, "Failed to initialize wired device");
        }
    }
}

fn handle_wifi_removed(device_path: &str, wifi: &Property<Option<Arc<Wifi>>>) {
    let Some(current) = wifi.get() else { return };

    if current.device.core.object_path.as_str() == device_path {
        debug!(path = %device_path, "WiFi device removed");
        wifi.set(None);
    }
}

fn handle_wired_removed(device_path: &str, wired: &Property<Option<Arc<Wired>>>) {
    let Some(current) = wired.get() else { return };

    if current.device.core.object_path.as_str() == device_path {
        debug!(path = %device_path, "Wired device removed");
        wired.set(None);
    }
}

fn update_primary_connection(
    nm_primary: &OwnedObjectPath,
    wifi: &Property<Option<Arc<Wifi>>>,
    wired: &Property<Option<Arc<Wired>>>,
    primary: &Property<ConnectionType>,
) {
    let nm_primary_str = nm_primary.as_str();

    if nm_primary_str.is_empty() || nm_primary_str == "/" {
        primary.set(ConnectionType::Unknown);
        return;
    }

    if let Some(wifi_service) = wifi.get() {
        let active_conn = wifi_service.device.core.active_connection.get();
        if active_conn.as_str() == nm_primary_str {
            primary.set(ConnectionType::Wifi);
            return;
        }
    }

    if let Some(wired_service) = wired.get() {
        let active_conn = wired_service.device.core.active_connection.get();
        if active_conn.as_str() == nm_primary_str {
            primary.set(ConnectionType::Wired);
            return;
        }
    }

    primary.set(ConnectionType::Unknown);
}
