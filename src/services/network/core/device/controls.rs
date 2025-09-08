use std::collections::HashMap;

use tracing::instrument;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::services::network::{error::Error, proxy::devices::DeviceProxy};

use super::types::AppliedConnection;

pub(super) struct DeviceControls;

impl DeviceControls {
    #[instrument(
        skip(connection),
        fields(device = %path, managed = managed),
        err
    )]
    pub(super) async fn set_managed(
        connection: &Connection,
        path: &OwnedObjectPath,
        managed: bool,
    ) -> Result<(), Error> {
        let proxy = DeviceProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy
            .set_managed(managed)
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "set_managed",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    #[instrument(
        skip(connection),
        fields(device = %path, autoconnect = autoconnect),
        err
    )]
    pub(super) async fn set_autoconnect(
        connection: &Connection,
        path: &OwnedObjectPath,
        autoconnect: bool,
    ) -> Result<(), Error> {
        let proxy = DeviceProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy
            .set_autoconnect(autoconnect)
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "set_autoconnect",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    #[instrument(
        skip(connection, connection_settings),
        fields(device = %path, version = version_id, flags = flags),
        err
    )]
    pub(super) async fn reapply(
        connection: &Connection,
        path: &OwnedObjectPath,
        connection_settings: HashMap<String, HashMap<String, OwnedValue>>,
        version_id: u64,
        flags: u32,
    ) -> Result<(), Error> {
        let proxy = DeviceProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy
            .reapply(connection_settings, version_id, flags)
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "reapply",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    #[instrument(
        skip(connection),
        fields(device = %path, flags = flags),
        err
    )]
    pub(super) async fn get_applied_connection(
        connection: &Connection,
        path: &OwnedObjectPath,
        flags: u32,
    ) -> Result<AppliedConnection, Error> {
        let proxy = DeviceProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy
            .get_applied_connection(flags)
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "get_applied_connection",
                reason: e.to_string(),
            })
    }

    #[instrument(skip(connection), fields(device = %path), err)]
    pub(super) async fn disconnect(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<(), Error> {
        let proxy = DeviceProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy
            .disconnect()
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "disconnect",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    #[instrument(skip(connection), fields(device = %path), err)]
    pub(super) async fn delete(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<(), Error> {
        let proxy = DeviceProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy.delete().await.map_err(|e| Error::OperationFailed {
            operation: "delete",
            reason: e.to_string(),
        })?;

        Ok(())
    }
}
