use std::sync::Arc;

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{
    error::Error, proxy::manager::NetworkManagerProxy, service::NetworkService,
    types::connectivity::ConnectionType, wifi::Wifi, wired::Wired,
};

impl ServiceMonitoring for NetworkService {
    type Error = Error;

    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        spawn_primary_monitoring(
            &self.zbus_connection,
            self.wifi.clone(),
            self.wired.clone(),
            self.primary.clone(),
            self.cancellation_token.child_token(),
        )
        .await
    }
}

async fn spawn_primary_monitoring(
    connection: &Connection,
    wifi: Option<Arc<Wifi>>,
    wired: Option<Arc<Wired>>,
    primary: Property<ConnectionType>,
    cancellation_token: CancellationToken,
) -> Result<(), Error> {
    let nm_proxy = NetworkManagerProxy::new(connection)
        .await
        .map_err(Error::DbusError)?;

    let primary_connection = nm_proxy.primary_connection().await?;
    update_primary_connection(primary_connection, &wifi, &wired, &primary).await;

    let mut primary_changed = nm_proxy.receive_primary_connection_changed().await;

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("NetworkMonitoring primary monitoring cancelled");
                    return;
                }
                Some(change) = primary_changed.next() => {
                    if let Ok(new_primary_connection) = change.get().await {
                        debug!("Primary Connection: {new_primary_connection}");
                        update_primary_connection(
                            new_primary_connection,
                            &wifi,
                            &wired,
                            &primary,
                        )
                        .await;
                    }
                }
            }
        }
    });

    Ok(())
}

async fn update_primary_connection(
    connection: OwnedObjectPath,
    wifi: &Option<Arc<Wifi>>,
    wired: &Option<Arc<Wired>>,
    primary: &Property<ConnectionType>,
) {
    if let Some(wifi_service) = wifi
        && wifi_service.device.core.active_connection.get().as_str() == connection.as_str()
    {
        primary.set(ConnectionType::Wifi);
        return;
    }

    if let Some(wired_service) = wired
        && wired_service.device.core.active_connection.get().as_str() == connection.as_str()
    {
        primary.set(ConnectionType::Wired);
        return;
    }

    primary.set(ConnectionType::Unknown);
}
