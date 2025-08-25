//! NetworkManager Settings interface.

use std::collections::HashMap;

use zbus::{
    proxy,
    zvariant::{OwnedObjectPath, OwnedValue},
};

pub mod connection;

/// Connection Settings Profile Manager.
///
/// Manages saved network connection profiles.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Settings",
    default_path = "/org/freedesktop/NetworkManager/Settings"
)]
pub trait Settings {
    /// List the saved network connections known to NetworkManager.
    ///
    /// # Returns
    /// List of connections object paths
    fn list_connections(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// Retrieve the object path of a connection, given that connection's UUID.
    ///
    /// # Arguments
    /// * `uuid` - The UUID to find the connection object path for
    ///
    /// # Returns
    /// The connection object path
    fn get_connection_by_uuid(&self, uuid: &str) -> zbus::Result<OwnedObjectPath>;

    /// Add new connection and save it to disk.
    ///
    /// # Arguments
    /// * `connection` - Connection settings and properties
    ///
    /// # Returns
    /// Object path of the new connection that was just added
    fn add_connection(
        &self,
        connection: HashMap<String, HashMap<String, OwnedValue>>,
    ) -> zbus::Result<OwnedObjectPath>;

    /// Add new connection but do not save it to disk immediately.
    ///
    /// # Arguments
    /// * `connection` - Connection settings and properties
    ///
    /// # Returns
    /// Object path of the new connection that was just added
    fn add_connection_unsaved(
        &self,
        connection: HashMap<String, HashMap<String, OwnedValue>>,
    ) -> zbus::Result<OwnedObjectPath>;

    /// Add a new connection profile.
    ///
    /// # Arguments
    /// * `settings` - New connection settings
    /// * `flags` - NMSettingsAddConnection2Flags
    /// * `args` - Optional arguments
    ///
    /// # Returns
    /// * Connection path
    /// * Result dictionary
    fn add_connection2(
        &self,
        settings: HashMap<String, HashMap<String, OwnedValue>>,
        flags: u32,
        args: HashMap<String, OwnedValue>,
    ) -> zbus::Result<(OwnedObjectPath, HashMap<String, OwnedValue>)>;

    /// Loads or reloads the indicated connections from disk.
    ///
    /// # Arguments
    /// * `filenames` - Array of paths to on-disk connection profiles
    ///
    /// # Returns
    /// * Success status
    /// * Paths of connection objects that failed to load
    fn load_connections(&self, filenames: Vec<String>) -> zbus::Result<(bool, Vec<String>)>;

    /// Tells NetworkManager to reload all connection files from disk.
    ///
    /// # Returns
    /// Success status
    fn reload_connections(&self) -> zbus::Result<bool>;

    /// Save the hostname to persistent configuration.
    ///
    /// # Arguments
    /// * `hostname` - The hostname to save to persistent configuration.
    ///                If blank, the persistent hostname is cleared.
    fn save_hostname(&self, hostname: &str) -> zbus::Result<()>;

    /// List of object paths of available network connection profiles.
    #[zbus(property)]
    fn connections(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// The machine hostname stored in persistent configuration.
    #[zbus(property)]
    fn hostname(&self) -> zbus::Result<String>;

    /// If true, adding and modifying connections is supported.
    #[zbus(property)]
    fn can_modify(&self) -> zbus::Result<bool>;

    /// The version of the settings. This is incremented whenever the profile changes
    /// and can be used to detect concurrent modifications.
    #[zbus(property)]
    fn version_id(&self) -> zbus::Result<u64>;

    /// Emitted when a new connection has been added.
    #[zbus(signal)]
    fn new_connection(&self, connection: OwnedObjectPath) -> zbus::Result<()>;

    /// Emitted when a connection is no longer available.
    #[zbus(signal)]
    fn connection_removed(&self, connection: OwnedObjectPath) -> zbus::Result<()>;
}
