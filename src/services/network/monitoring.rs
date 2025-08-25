use std::sync::Arc;

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{ConnectionType, NetworkError, NetworkManagerProxy, Wifi, Wired};
use crate::services::common::Property;

/// Handles ongoing monitoring of network devices and connections.
pub(crate) struct NetworkMonitoring;

impl NetworkMonitoring {
    pub(crate) async fn start(
        connection: &Connection,
        wifi: Option<Arc<Wifi>>,
        wired: Option<Arc<Wired>>,
        primary: Property<ConnectionType>,
        cancellation_token: CancellationToken,
    ) -> Result<(), NetworkError> {
        Self::spawn_primary_monitoring(connection, wifi, wired, primary, cancellation_token)
            .await?;

        Ok(())
    }

    async fn spawn_primary_monitoring(
        connection: &Connection,
        wifi: Option<Arc<Wifi>>,
        wired: Option<Arc<Wired>>,
        primary: Property<ConnectionType>,
        cancellation_token: CancellationToken,
    ) -> Result<(), NetworkError> {
        let nm_proxy = NetworkManagerProxy::new(connection)
            .await
            .map_err(NetworkError::DbusError)?;

        let primary_connection = nm_proxy.primary_connection().await?;
        Self::update_primary_connection(primary_connection, &wifi, &wired, &primary).await;

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
                            Self::update_primary_connection(
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
            && wifi_service.active_connection.get().as_str() == connection.as_str()
        {
            primary.set(ConnectionType::Wifi);
            return;
        }

        if let Some(wired_service) = wired
            && wired_service.active_connection.get().as_str() == connection.as_str()
        {
            primary.set(ConnectionType::Wired);
            return;
        }

        primary.set(ConnectionType::Unknown);
    }
}
