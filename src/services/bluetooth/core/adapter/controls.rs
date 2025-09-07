use std::collections::HashMap;

use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, Value},
};

use crate::services::bluetooth::{
    error::BluetoothError, proxy::adapter::Adapter1Proxy, types::adapter::DiscoveryFilter,
};

pub(crate) struct AdapterControls;

impl AdapterControls {
    pub(super) async fn set_alias(
        connection: &Connection,
        path: &OwnedObjectPath,
        alias: &str,
    ) -> Result<(), BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path).await?;

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
        let proxy = Adapter1Proxy::new(connection, path).await?;

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
        let proxy = Adapter1Proxy::new(connection, path).await?;

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
        let proxy = Adapter1Proxy::new(connection, path).await?;

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
        let proxy = Adapter1Proxy::new(connection, path).await?;

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
        let proxy = Adapter1Proxy::new(connection, path).await?;

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
        let proxy = Adapter1Proxy::new(connection, path).await?;

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
        let proxy = Adapter1Proxy::new(connection, path).await?;

        proxy
            .set_discovery_filter(discovery_filter)
            .await
            .map_err(|e| BluetoothError::OperationFailed {
                operation: "set_discovery_filter",
                reason: e.to_string(),
            })
    }

    pub(super) async fn start_discovery(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<(), BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path).await?;

        proxy
            .start_discovery()
            .await
            .map_err(|e| BluetoothError::OperationFailed {
                operation: "start_discovery",
                reason: e.to_string(),
            })
    }

    pub(super) async fn stop_discovery(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<(), BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path).await?;

        proxy
            .stop_discovery()
            .await
            .map_err(|e| BluetoothError::OperationFailed {
                operation: "stop_discovery",
                reason: e.to_string(),
            })
    }

    pub(super) async fn remove_device(
        connection: &Connection,
        path: &OwnedObjectPath,
        device_path: &OwnedObjectPath,
    ) -> Result<(), BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path).await?;

        proxy
            .remove_device(device_path)
            .await
            .map_err(|e| BluetoothError::OperationFailed {
                operation: "remove_device",
                reason: e.to_string(),
            })
    }

    pub(super) async fn get_discovery_filters(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<Vec<String>, BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path).await?;

        proxy
            .get_discovery_filters()
            .await
            .map_err(BluetoothError::DbusError)
    }

    pub(super) async fn connect_device(
        connection: &Connection,
        path: &OwnedObjectPath,
        properties: HashMap<String, Value<'_>>,
    ) -> Result<OwnedObjectPath, BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, path).await?;

        proxy
            .connect_device(properties)
            .await
            .map_err(|e| BluetoothError::OperationFailed {
                operation: "connect_device",
                reason: e.to_string(),
            })
    }
}
