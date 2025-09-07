use std::collections::HashMap;

use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::services::network::{error::NetworkError, proxy::devices::DeviceProxy};

pub(super) struct DeviceControls;

impl DeviceControls {
    pub(super) async fn set_managed(
        connection: &Connection,
        path: &OwnedObjectPath,
        managed: bool,
    ) -> Result<(), NetworkError> {
        let proxy = DeviceProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        proxy
            .set_managed(managed)
            .await
            .map_err(|e| NetworkError::OperationFailed {
                operation: "set_managed",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    pub(super) async fn set_autoconnect(
        connection: &Connection,
        path: &OwnedObjectPath,
        autoconnect: bool,
    ) -> Result<(), NetworkError> {
        let proxy = DeviceProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        proxy
            .set_autoconnect(autoconnect)
            .await
            .map_err(|e| NetworkError::OperationFailed {
                operation: "set_autoconnect",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    pub(super) async fn reapply(
        connection: &Connection,
        path: &OwnedObjectPath,
        connection_settings: HashMap<String, HashMap<String, OwnedValue>>,
        version_id: u64,
        flags: u32,
    ) -> Result<(), NetworkError> {
        let proxy = DeviceProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        proxy
            .reapply(connection_settings, version_id, flags)
            .await
            .map_err(|e| NetworkError::OperationFailed {
                operation: "reapply",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    pub(super) async fn get_applied_connection(
        connection: &Connection,
        path: &OwnedObjectPath,
        flags: u32,
    ) -> Result<(HashMap<String, HashMap<String, OwnedValue>>, u64), NetworkError> {
        let proxy = DeviceProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        proxy
            .get_applied_connection(flags)
            .await
            .map_err(|e| NetworkError::OperationFailed {
                operation: "get_applied_connection",
                reason: e.to_string(),
            })
    }

    pub(super) async fn disconnect(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<(), NetworkError> {
        let proxy = DeviceProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        proxy
            .disconnect()
            .await
            .map_err(|e| NetworkError::OperationFailed {
                operation: "disconnect",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    pub(super) async fn delete(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<(), NetworkError> {
        let proxy = DeviceProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        proxy
            .delete()
            .await
            .map_err(|e| NetworkError::OperationFailed {
                operation: "delete",
                reason: e.to_string(),
            })?;

        Ok(())
    }
}
