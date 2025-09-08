use std::collections::HashMap;

use tracing::instrument;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::services::network::{
    error::Error, proxy::settings::connection::SettingsConnectionProxy,
};

pub(super) struct ConnectionSettingsControls;

impl ConnectionSettingsControls {
    #[instrument(
        skip(connection, properties),
        fields(path = %path),
        err
    )]
    pub(super) async fn update(
        connection: &Connection,
        path: &OwnedObjectPath,
        properties: HashMap<String, HashMap<String, OwnedValue>>,
    ) -> Result<(), Error> {
        let proxy = SettingsConnectionProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy
            .update(properties)
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "update_connection",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    #[instrument(
        skip(connection, properties),
        fields(path = %path),
        err
    )]
    pub(super) async fn update_unsaved(
        connection: &Connection,
        path: &OwnedObjectPath,
        properties: HashMap<String, HashMap<String, OwnedValue>>,
    ) -> Result<(), Error> {
        let proxy = SettingsConnectionProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy
            .update_unsaved(properties)
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "update_connection_unsaved",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    #[instrument(skip(connection), fields(path = %path), err)]
    pub(super) async fn delete(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<(), Error> {
        let proxy = SettingsConnectionProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy.delete().await.map_err(|e| Error::OperationFailed {
            operation: "delete_connection",
            reason: e.to_string(),
        })?;

        Ok(())
    }

    #[instrument(skip(connection), fields(path = %path), err)]
    pub(super) async fn get_settings(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<HashMap<String, HashMap<String, OwnedValue>>, Error> {
        let proxy = SettingsConnectionProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy
            .get_settings()
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "get_settings",
                reason: e.to_string(),
            })
    }

    #[instrument(
        skip(connection),
        fields(path = %path, setting = %setting_name),
        err
    )]
    pub(super) async fn get_secrets(
        connection: &Connection,
        path: &OwnedObjectPath,
        setting_name: &str,
    ) -> Result<HashMap<String, HashMap<String, OwnedValue>>, Error> {
        let proxy = SettingsConnectionProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy
            .get_secrets(setting_name)
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "get_secrets",
                reason: e.to_string(),
            })
    }

    #[instrument(skip(connection), fields(path = %path), err)]
    pub(super) async fn clear_secrets(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<(), Error> {
        let proxy = SettingsConnectionProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy
            .clear_secrets()
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "clear_secrets",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    #[instrument(skip(connection), fields(path = %path), err)]
    pub(super) async fn save(connection: &Connection, path: &OwnedObjectPath) -> Result<(), Error> {
        let proxy = SettingsConnectionProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy.save().await.map_err(|e| Error::OperationFailed {
            operation: "save_connection",
            reason: e.to_string(),
        })?;

        Ok(())
    }

    #[instrument(
        skip(connection, settings, args),
        fields(path = %path, flags = flags),
        err
    )]
    pub(super) async fn update2(
        connection: &Connection,
        path: &OwnedObjectPath,
        settings: HashMap<String, HashMap<String, OwnedValue>>,
        flags: u32,
        args: HashMap<String, OwnedValue>,
    ) -> Result<HashMap<String, OwnedValue>, Error> {
        let proxy = SettingsConnectionProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy
            .update2(settings, flags, args)
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "update2_connection",
                reason: e.to_string(),
            })
    }
}
