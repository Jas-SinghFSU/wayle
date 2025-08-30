use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::services::bluetooth::{BluetoothError, proxy::Adapter1Proxy, types::DiscoveryFilter};

pub(crate) struct AdapterControls;

impl AdapterControls {
    pub(super) async fn set_alias(
        connection: &Connection,
        path: &OwnedObjectPath,
        alias: &str,
    ) -> Result<(), BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path.clone()).await?;

        proxy
            .set_alias(alias)
            .await
            .map_err(|e| BluetoothError::OperationFailed {
                operation: "set_alias",
                reason: e.to_string(),
            })
    }

    pub(super) async fn set_connectable(
        connection: &Connection,
        path: &OwnedObjectPath,
        connectable: bool,
    ) -> Result<(), BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path.clone()).await?;

        proxy
            .set_connectable(connectable)
            .await
            .map_err(|e| BluetoothError::OperationFailed {
                operation: "set_connectable",
                reason: e.to_string(),
            })
    }

    pub(super) async fn set_powered(
        connection: &Connection,
        path: &OwnedObjectPath,
        powered: bool,
    ) -> Result<(), BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path.clone()).await?;

        proxy
            .set_powered(powered)
            .await
            .map_err(|e| BluetoothError::OperationFailed {
                operation: "set_powered",
                reason: e.to_string(),
            })
    }

    pub(super) async fn set_discoverable(
        connection: &Connection,
        path: &OwnedObjectPath,
        discoverable: bool,
    ) -> Result<(), BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path.clone()).await?;

        proxy
            .set_discoverable(discoverable)
            .await
            .map_err(|e| BluetoothError::OperationFailed {
                operation: "set_discoverable",
                reason: e.to_string(),
            })
    }

    pub(super) async fn set_discoverable_timeout(
        connection: &Connection,
        path: &OwnedObjectPath,
        discoverable_timeout: u32,
    ) -> Result<(), BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path.clone()).await?;

        proxy
            .set_discoverable_timeout(discoverable_timeout)
            .await
            .map_err(|e| BluetoothError::OperationFailed {
                operation: "set_discoverable_timeout",
                reason: e.to_string(),
            })
    }

    pub(super) async fn set_pairable(
        connection: &Connection,
        path: &OwnedObjectPath,
        pairable: bool,
    ) -> Result<(), BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path.clone()).await?;

        proxy
            .set_pairable(pairable)
            .await
            .map_err(|e| BluetoothError::OperationFailed {
                operation: "set_pairable",
                reason: e.to_string(),
            })
    }

    pub(super) async fn set_pairable_timeout(
        connection: &Connection,
        path: &OwnedObjectPath,
        pairable_timeout: u32,
    ) -> Result<(), BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path.clone()).await?;

        proxy
            .set_pairable_timeout(pairable_timeout)
            .await
            .map_err(|e| BluetoothError::OperationFailed {
                operation: "set_pairable_timeout",
                reason: e.to_string(),
            })
    }

    pub(super) async fn set_discovery_filter(
        connection: &Connection,
        path: &OwnedObjectPath,
        discovery_filter: DiscoveryFilter<'_>,
    ) -> Result<(), BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path.clone()).await?;

        proxy
            .set_discovery_filter(discovery_filter)
            .await
            .map_err(|e| BluetoothError::OperationFailed {
                operation: "set_discovery_filter",
                reason: e.to_string(),
            })
    }
}
