use std::collections::HashMap;

use zbus::{
    Result, proxy,
    zvariant::{OwnedObjectPath, OwnedValue},
};

/// Type alias for the complex return type of `get_managed_objects()`.
///
/// This represents a nested structure where:
/// - **Outer HashMap key**: Object path (e.g., `/org/bluez/hci0`)
/// - **Outer HashMap value**: Interface map for that object
///   - **Interface map key**: Interface name (e.g., `org.bluez.Adapter1`)
///   - **Interface map value**: Property map for that interface
///     - **Property map key**: Property name (e.g., `Address`, `Powered`)
///     - **Property map value**: Property value (e.g., `"00:11:22:33:44:55"`, `true`)
///
/// # Example structure for BlueZ:
/// ```text
/// {
///   "/org/bluez/hci0": {
///     "org.bluez.Adapter1": {
///       "Address": "00:11:22:33:44:55",
///       "Name": "hostname",
///       "Powered": true,
///       "Discoverable": false
///     }
///   },
///   "/org/bluez/hci0/dev_AA_BB_CC_DD_EE_FF": {
///     "org.bluez.Device1": {
///       "Address": "AA:BB:CC:DD:EE:FF",
///       "Name": "My Headphones",
///       "Connected": true
///     }
///   }
/// }
/// ```
pub type ManagedObjects = HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>>;

/// Proxy for the org.freedesktop.DBus.ObjectManager interface.
///
/// Standard D-Bus ObjectManager interface (D-Bus specification v0.17+) for discovering
/// all managed objects in a service's object tree in a single method call.
///
/// This interface is implemented by services like BlueZ, NetworkManager, UDisks2, etc.
/// to provide efficient object discovery and monitoring.
///
/// Interface: `org.freedesktop.DBus.ObjectManager`
/// 
/// # Usage Pattern
/// 1. Connect to the service's ObjectManager (typically at root path `/`)
/// 2. Call `get_managed_objects()` to get all objects and their properties
/// 3. Filter objects by the interfaces you're interested in
/// 4. Monitor `interfaces_added` and `interfaces_removed` signals for changes
#[proxy(interface = "org.freedesktop.DBus.ObjectManager")]
pub trait ObjectManager {
    /// Gets all managed objects and their interfaces in the service's object tree.
    ///
    /// Returns all objects that are children of the ObjectManager's object path.
    /// Each object includes all its interfaces and their current properties,
    /// equivalent to calling `org.freedesktop.DBus.Properties.GetAll()` on each
    /// interface of each object.
    ///
    /// # Returns
    /// A [`ManagedObjects`] dictionary where:
    /// - Keys are object paths (children of this ObjectManager's path)
    /// - Values are interface maps containing all properties for each interface
    ///
    /// # D-Bus Method Signature
    /// ```text
    /// GetManagedObjects() -> a{oa{sa{sv}}}
    /// ```
    async fn get_managed_objects(&self) -> Result<ManagedObjects>;

    /// Signal emitted when new objects are added or existing objects gain interfaces.
    ///
    /// Contains all properties for the added interfaces, eliminating the need for
    /// separate `Properties.GetAll()` calls.
    ///
    /// # Parameters
    /// - `object_path`: Path of the object that was added or gained interfaces
    /// - `interfaces_and_properties`: Map of interface names to their properties
    #[zbus(signal)]
    fn interfaces_added(
        &self,
        object_path: zbus::zvariant::ObjectPath<'_>,
        interfaces_and_properties: HashMap<String, HashMap<String, zbus::zvariant::Value<'_>>>,
    ) -> zbus::Result<()>;

    /// Signal emitted when objects are removed or lose interfaces.
    ///
    /// # Parameters  
    /// - `object_path`: Path of the object that was removed or lost interfaces
    /// - `interfaces`: Array of interface names that were removed
    #[zbus(signal)]
    fn interfaces_removed(
        &self,
        object_path: zbus::zvariant::ObjectPath<'_>,
        interfaces: Vec<String>,
    ) -> zbus::Result<()>;
}
