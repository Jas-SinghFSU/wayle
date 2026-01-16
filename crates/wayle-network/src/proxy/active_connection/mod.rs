//! NetworkManager Active Connection interfaces.

use zbus::{proxy, zvariant::OwnedObjectPath};

pub mod vpn;

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Connection.Active"
)]
pub(crate) trait ConnectionActive {
    /// The path of the connection object that this ActiveConnection is using.
    #[zbus(property)]
    fn connection(&self) -> zbus::Result<OwnedObjectPath>;

    /// A specific object associated with the active connection.
    /// This property reflects the specific object used during connection activation,
    /// and will not change over the lifetime of the ActiveConnection once set.
    #[zbus(property)]
    fn specific_object(&self) -> zbus::Result<OwnedObjectPath>;

    /// The ID of the connection, provided for convenience.
    #[zbus(property)]
    fn id(&self) -> zbus::Result<String>;

    /// The UUID of the connection, provided for convenience.
    #[zbus(property)]
    fn uuid(&self) -> zbus::Result<String>;

    /// The type of the connection, provided for convenience.
    #[zbus(property, name = "Type")]
    fn type_(&self) -> zbus::Result<String>;

    /// Array of object paths representing devices which are part of this active connection.
    #[zbus(property)]
    fn devices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// The state of this active connection.
    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;

    /// The state flags of this active connection.
    #[zbus(property)]
    fn state_flags(&self) -> zbus::Result<u32>;

    /// Whether this active connection is the default IPv4 connection.
    #[zbus(property)]
    fn default(&self) -> zbus::Result<bool>;

    /// Object path of the Ip4Config object describing the configuration of the connection.
    #[zbus(property)]
    fn ip4_config(&self) -> zbus::Result<OwnedObjectPath>;

    /// Object path of the Dhcp4Config object describing the DHCP options.
    #[zbus(property)]
    fn dhcp4_config(&self) -> zbus::Result<OwnedObjectPath>;

    /// Whether this active connection is the default IPv6 connection.
    #[zbus(property)]
    fn default6(&self) -> zbus::Result<bool>;

    /// Object path of the Ip6Config object describing the configuration of the connection.
    #[zbus(property)]
    fn ip6_config(&self) -> zbus::Result<OwnedObjectPath>;

    /// Object path of the Dhcp6Config object describing the DHCP options.
    #[zbus(property)]
    fn dhcp6_config(&self) -> zbus::Result<OwnedObjectPath>;

    /// Whether this active connection is also a VPN connection.
    #[zbus(property)]
    fn vpn(&self) -> zbus::Result<bool>;

    /// The path to the master device if the connection is a slave.
    #[zbus(property)]
    fn master(&self) -> zbus::Result<OwnedObjectPath>;

    /// The path to the controller device if the connection is a port.
    #[zbus(property)]
    fn controller(&self) -> zbus::Result<OwnedObjectPath>;

    /// Emitted when the active connection changes state.
    #[zbus(signal, name = "StateChanged")]
    fn active_connection_state_changed(&self, state: u32, reason: u32) -> zbus::Result<()>;
}
