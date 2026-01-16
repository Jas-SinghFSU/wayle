//! NetworkManager Settings Connection interface.

use std::collections::HashMap;

use zbus::{proxy, zvariant::OwnedValue};

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Settings.Connection"
)]
pub(crate) trait SettingsConnection {
    /// Update the connection with new settings and properties.
    ///
    /// # Arguments
    /// * `properties` - New connection settings
    fn update(&self, properties: HashMap<String, HashMap<String, OwnedValue>>) -> zbus::Result<()>;

    /// Update the connection with new settings and properties but do not immediately save the connection to disk.
    ///
    /// # Arguments
    /// * `properties` - New connection settings
    fn update_unsaved(
        &self,
        properties: HashMap<String, HashMap<String, OwnedValue>>,
    ) -> zbus::Result<()>;

    /// Delete the connection from backing storage.
    fn delete(&self) -> zbus::Result<()>;

    /// Get the settings maps describing this network configuration.
    ///
    /// # Returns
    /// Connection settings (without secrets)
    fn get_settings(&self) -> zbus::Result<HashMap<String, HashMap<String, OwnedValue>>>;

    /// Get the secrets belonging to this network configuration.
    ///
    /// # Arguments
    /// * `setting_name` - Name of the setting to get secrets for
    ///
    /// # Returns
    /// Nested settings maps containing secrets
    fn get_secrets(
        &self,
        setting_name: &str,
    ) -> zbus::Result<HashMap<String, HashMap<String, OwnedValue>>>;

    /// Clear the secrets belonging to this network connection profile.
    fn clear_secrets(&self) -> zbus::Result<()>;

    /// Saves a "dirty" connection to persistent storage.
    fn save(&self) -> zbus::Result<()>;

    /// Update the connection and save it to disk.
    ///
    /// # Arguments
    /// * `settings` - Optional settings to update
    /// * `flags` - NMSettingsUpdate2Flags
    /// * `args` - Optional arguments
    ///
    /// # Returns
    /// Currently no additional results
    fn update2(
        &self,
        settings: HashMap<String, HashMap<String, OwnedValue>>,
        flags: u32,
        args: HashMap<String, OwnedValue>,
    ) -> zbus::Result<HashMap<String, OwnedValue>>;

    /// If set, indicates that the in-memory state of the connection does not match the on-disk state.
    #[zbus(property)]
    fn unsaved(&self) -> zbus::Result<bool>;

    /// Additional flags of the connection profile.
    #[zbus(property)]
    fn flags(&self) -> zbus::Result<u32>;

    /// File that stores the connection in case the connection is file-backed.
    #[zbus(property)]
    fn filename(&self) -> zbus::Result<String>;

    /// Emitted when any property of any settings object within this Connection has changed.
    #[zbus(signal)]
    fn properties_changed(&self, properties: HashMap<String, OwnedValue>) -> zbus::Result<()>;

    /// Emitted when the connection is updated.
    #[zbus(signal)]
    fn updated(&self) -> zbus::Result<()>;

    /// Emitted when the connection is removed.
    #[zbus(signal)]
    fn removed(&self) -> zbus::Result<()>;
}
