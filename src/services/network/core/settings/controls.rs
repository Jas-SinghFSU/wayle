use std::collections::HashMap;

use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::services::network::{NMSettingsAddConnection2Flags, NetworkError, SettingsProxy};

pub(super) struct SettingsController;

impl SettingsController {
    pub(super) async fn list_connections(
        zbus_connection: &Connection,
    ) -> Result<Vec<OwnedObjectPath>, NetworkError> {
        let settings_proxy = SettingsProxy::new(zbus_connection).await?;
        let connections = settings_proxy.list_connections().await?;

        Ok(connections)
    }

    pub(super) async fn get_connection_by_uuid(
        zbus_connection: &Connection,
        uuid: &str,
    ) -> Result<OwnedObjectPath, NetworkError> {
        let settings_proxy = SettingsProxy::new(zbus_connection).await?;
        let connection = settings_proxy.get_connection_by_uuid(uuid).await?;

        Ok(connection)
    }

    pub(super) async fn add_connection(
        zbus_connection: &Connection,
        connection: HashMap<String, HashMap<String, OwnedValue>>,
    ) -> Result<OwnedObjectPath, NetworkError> {
        let settings_proxy = SettingsProxy::new(zbus_connection).await?;
        let created_connection = settings_proxy.add_connection_unsaved(connection).await?;

        Ok(created_connection)
    }

    pub(super) async fn add_connection2(
        zbus_connection: &Connection,
        settings: HashMap<String, HashMap<String, OwnedValue>>,
        flags: NMSettingsAddConnection2Flags,
        args: HashMap<String, OwnedValue>,
    ) -> Result<(OwnedObjectPath, HashMap<String, OwnedValue>), NetworkError> {
        let settings_proxy = SettingsProxy::new(zbus_connection).await?;

        let (path, result) = settings_proxy
            .add_connection2(settings, flags.bits(), args)
            .await
            .map_err(NetworkError::DbusError)?;

        Ok((path, result))
    }

    pub(super) async fn load_connections(
        zbus_connection: &Connection,
        filenames: Vec<String>,
    ) -> Result<(bool, Vec<String>), NetworkError> {
        let settings_proxy = SettingsProxy::new(zbus_connection).await?;

        let (status, failures) = settings_proxy
            .load_connections(filenames)
            .await
            .map_err(NetworkError::DbusError)?;

        Ok((status, failures))
    }

    pub(super) async fn reload_connections(
        zbus_connection: &Connection,
    ) -> Result<bool, NetworkError> {
        let settings_proxy = SettingsProxy::new(zbus_connection).await?;

        let status = settings_proxy
            .reload_connections()
            .await
            .map_err(NetworkError::DbusError)?;

        Ok(status)
    }

    pub(super) async fn save_hostname(
        zbus_connection: &Connection,
        hostname: &str,
    ) -> Result<(), NetworkError> {
        let settings_proxy = SettingsProxy::new(zbus_connection).await?;

        settings_proxy
            .save_hostname(hostname)
            .await
            .map_err(NetworkError::DbusError)
    }
}
