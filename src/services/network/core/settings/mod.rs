mod controls;
mod monitoring;
mod types;

use std::{collections::HashMap, sync::Arc};

use controls::SettingsController;
use futures::{Stream, StreamExt, future::join_all};
use tokio_util::sync::CancellationToken;
pub(crate) use types::{LiveSettingsParams, SettingsParams};
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use super::{
    access_point::types::Ssid,
    settings_connection::{ConnectionSettings, ConnectionSettingsParams},
};
use crate::{
    services::{
        common::Property,
        network::{
            error::Error, proxy::settings::SettingsProxy,
            types::flags::NMSettingsAddConnection2Flags,
        },
        traits::{ModelMonitoring, Reactive},
    },
    unwrap_bool, unwrap_string, unwrap_u64, unwrap_vec,
};

/// Connection Settings Profile Manager.
///
/// The Settings interface allows clients to view and administrate
/// the connections stored and used by NetworkManager.
#[derive(Debug, Clone)]
pub struct Settings {
    pub(crate) zbus_connection: Connection,
    pub(crate) cancellation_token: Option<CancellationToken>,
    /// List of object paths of available network connection profiles.
    pub connections: Property<Vec<ConnectionSettings>>,
    /// The machine hostname stored in persistent configuration.
    pub hostname: Property<String>,
    /// If true, adding and modifying connections is supported.
    pub can_modify: Property<bool>,
    /// The version of the settings. This is incremented whenever the profile
    /// changes and can be used to detect concurrent modifications. Since: 1.44
    pub version_id: Property<u64>,
}

impl Reactive for Settings {
    type Context<'a> = SettingsParams<'a>;
    type LiveContext<'a> = LiveSettingsParams<'a>;
    type Error = Error;

    async fn get(params: Self::Context<'_>) -> Result<Self, Self::Error> {
        Self::from_connection(params.zbus_connection, None).await
    }

    async fn get_live(params: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let settings = Self::from_connection(
            params.zbus_connection,
            Some(params.cancellation_token.child_token()),
        )
        .await?;
        let settings = Arc::new(settings);

        settings.clone().start_monitoring().await?;

        Ok(settings)
    }
}

impl Settings {
    /// List the saved network connections known to NetworkManager.
    ///
    /// # Returns
    ///
    /// List of connection object paths.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::DbusError` if the DBus operation fails.
    pub async fn list_connections(&self) -> Result<Vec<OwnedObjectPath>, Error> {
        SettingsController::list_connections(&self.zbus_connection).await
    }

    /// Retrieve the object path of a connection, given that connection's UUID.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The UUID to find the connection object path for.
    ///
    /// # Returns
    ///
    /// The connection's object path.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::DbusError` if the DBus operation fails or connection not found.
    pub async fn get_connection_by_uuid(&self, uuid: &str) -> Result<OwnedObjectPath, Error> {
        SettingsController::get_connection_by_uuid(&self.zbus_connection, uuid).await
    }

    /// Add new connection and save it to disk.
    ///
    /// This operation does not start the network connection unless
    /// (1) device is idle and able to connect to the network described
    ///     by the new connection AND
    /// (2) the connection is allowed to be started automatically.
    ///
    /// # Arguments
    ///
    /// * `connection` - Connection settings and properties.
    ///
    /// # Returns
    ///
    /// Object path of the new connection that was just added.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::DbusError` if the DBus operation fails.
    pub async fn add_connection(
        &self,
        connection: HashMap<String, HashMap<String, OwnedValue>>,
    ) -> Result<OwnedObjectPath, Error> {
        SettingsController::add_connection(&self.zbus_connection, connection).await
    }

    /// Add new connection but do not save it to disk immediately.
    ///
    /// This operation does not start the network connection unless (1) device is idle
    /// and able to connect to the network described by the new connection, and (2) the
    /// connection is allowed to be started automatically. Use the 'Save' method on the
    /// connection to save these changes to disk.
    ///
    /// # Arguments
    ///
    /// * `connection` - Connection settings and properties.
    ///
    /// # Returns
    ///
    /// Object path of the new connection that was just added.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::DbusError` if the DBus operation fails.
    pub async fn add_connection_unsaved(
        &self,
        connection: HashMap<String, HashMap<String, OwnedValue>>,
    ) -> Result<OwnedObjectPath, Error> {
        SettingsController::add_connection_unsaved(&self.zbus_connection, connection).await
    }

    /// Add a new connection profile.
    ///
    /// AddConnection2 is an alternative to AddConnection and AddConnectionUnsaved.
    /// The new variant can do everything that the older variants could, and more.
    /// Its behavior is extensible via extra flags and args arguments.
    ///
    /// # Arguments
    ///
    /// * `settings` - Connection configuration as nested hashmaps. The outer map keys are
    ///   setting names like "connection", "802-3-ethernet", "ipv4", etc. The inner maps
    ///   contain the properties for each setting.
    /// * `flags` - Control how the connection is stored:
    ///   - `TO_DISK`: Persist the connection to disk
    ///   - `IN_MEMORY`: Keep the connection in memory only
    ///   - `BLOCK_AUTOCONNECT`: Prevent automatic connection until manually activated
    /// * `args` - Additional arguments:
    ///   - `"plugin"`: Specify storage backend like "keyfile" or "ifcfg-rh" (Since 1.38)
    /// # Returns
    ///
    /// Returns a tuple containing:
    /// - The DBus object path of the newly created connection
    /// - A result dictionary (currently empty but reserved for future use)
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::DbusError` if the DBus operation fails.
    pub async fn add_connection2(
        &self,
        settings: HashMap<String, HashMap<String, OwnedValue>>,
        flags: NMSettingsAddConnection2Flags,
        args: HashMap<String, OwnedValue>,
    ) -> Result<(OwnedObjectPath, HashMap<String, OwnedValue>), Error> {
        SettingsController::add_connection2(&self.zbus_connection, settings, flags, args).await
    }

    /// Loads or reloads the indicated connections from disk.
    ///
    /// You should call this after making changes directly to an on-disk
    /// connection file to make sure that NetworkManager sees the changes.
    /// As with AddConnection(), this operation does not necessarily start
    /// the network connection.
    ///
    /// # Arguments
    ///
    /// * `filenames` - Array of paths to on-disk connection profiles in directories monitored by NetworkManager
    ///
    /// # Returns
    ///
    /// Returns a tuple containing:
    /// - `status`: Success or failure of the operation as a whole. True if NetworkManager
    ///   at least tried to load the indicated connections, even if it did not succeed.
    ///   False if an error occurred before trying to load the connections (eg, permission denied).
    /// - `failures`: Paths of connection files that could not be loaded
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::DbusError` if the DBus operation fails.
    pub async fn load_connections(
        &self,
        filenames: Vec<String>,
    ) -> Result<(bool, Vec<String>), Error> {
        SettingsController::load_connections(&self.zbus_connection, filenames).await
    }

    /// Tells NetworkManager to reload all connection files from disk.
    ///
    /// Reloads all connection files from disk, including noticing any
    /// added or deleted connection files.
    ///
    /// # Returns
    ///
    /// This always returns true.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::DbusError` if the DBus operation fails.
    pub async fn reload_connections(&self) -> Result<bool, Error> {
        SettingsController::reload_connections(&self.zbus_connection).await
    }

    /// Save the hostname to persistent configuration.
    ///
    /// # Arguments
    ///
    /// * `hostname` - The hostname to save to persistent configuration.
    ///   If blank, the persistent hostname is cleared.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if the operations fails.
    pub async fn save_hostname(&self, hostname: &str) -> Result<(), Error> {
        SettingsController::save_hostname(&self.zbus_connection, hostname).await
    }

    /// Get saved connection profiles that match the given Ssid.
    ///
    /// Returns all connection profiles configured for the specified Ssid.
    /// A single Ssid may have multiple profiles with different configurations.
    pub async fn connections_for_ssid(&self, ssid: &Ssid) -> Vec<ConnectionSettings> {
        let mut matching = Vec::new();

        for connection in self.connections.get() {
            if connection.matches_ssid(ssid).await {
                matching.push(connection);
            }
        }

        matching
    }

    /// Get a reactive stream of saved connections for the given Ssid.
    ///
    /// Returns a stream that emits whenever connections are added, removed,
    /// or modified for the specified Ssid.
    pub fn connections_for_ssid_monitored(
        &self,
        ssid: Ssid,
    ) -> impl Stream<Item = Vec<ConnectionSettings>> + '_ {
        self.connections.watch().then(move |connections| {
            let ssid = ssid.clone();

            async move {
                let mut matching = Vec::new();

                for connection in connections {
                    if connection.matches_ssid(&ssid).await {
                        matching.push(connection);
                    }
                }

                matching
            }
        })
    }

    async fn from_connection(
        zbus_connection: &Connection,
        cancellation_token: Option<CancellationToken>,
    ) -> Result<Self, Error> {
        let settings_proxy = SettingsProxy::new(zbus_connection).await?;

        let (connections, hostname, can_modify, version_id) = tokio::join!(
            settings_proxy.connections(),
            settings_proxy.hostname(),
            settings_proxy.can_modify(),
            settings_proxy.version_id()
        );

        let connection_paths = unwrap_vec!(connections);

        let connection_futures = connection_paths.into_iter().map(|path| {
            ConnectionSettings::get(ConnectionSettingsParams {
                connection: zbus_connection,
                path,
            })
        });

        let connection_list: Vec<ConnectionSettings> = join_all(connection_futures)
            .await
            .into_iter()
            .flatten()
            .collect();

        Ok(Self {
            zbus_connection: zbus_connection.clone(),
            cancellation_token,
            connections: Property::new(connection_list),
            hostname: Property::new(unwrap_string!(hostname)),
            can_modify: Property::new(unwrap_bool!(can_modify)),
            version_id: Property::new(unwrap_u64!(version_id)),
        })
    }
}
