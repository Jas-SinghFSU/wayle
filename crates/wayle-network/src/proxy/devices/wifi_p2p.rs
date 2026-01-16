//! NetworkManager Wi-Fi P2P Device interface.

use std::collections::HashMap;

use zbus::{
    proxy,
    zvariant::{OwnedObjectPath, OwnedValue},
};

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.WifiP2P"
)]
pub(crate) trait DeviceWifiP2P {
    /// Start a Wi-Fi P2P find operation.
    ///
    /// # Arguments
    /// * `options` - Options dictionary (currently unused)
    fn start_find(&self, options: HashMap<String, OwnedValue>) -> zbus::Result<()>;

    /// Stop a find operation.
    fn stop_find(&self) -> zbus::Result<()>;

    /// Hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// The set of group objects representing groups.
    #[zbus(property)]
    fn groups(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// The set of P2P peer objects representing P2P peers.
    #[zbus(property)]
    fn peers(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// Emitted when a new Wi-Fi P2P peer is found.
    #[zbus(signal)]
    fn peer_added(&self, peer: OwnedObjectPath) -> zbus::Result<()>;

    /// Emitted when a Wi-Fi P2P peer is lost.
    #[zbus(signal)]
    fn peer_removed(&self, peer: OwnedObjectPath) -> zbus::Result<()>;
}
