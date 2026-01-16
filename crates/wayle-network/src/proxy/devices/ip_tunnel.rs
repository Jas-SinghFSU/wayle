//! NetworkManager IP Tunnel Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.IPTunnel"
)]
pub(crate) trait DeviceIPTunnel {
    /// The tunneling mode.
    #[zbus(property)]
    fn mode(&self) -> zbus::Result<u32>;

    /// The object path of the parent device.
    #[zbus(property)]
    fn parent(&self) -> zbus::Result<OwnedObjectPath>;

    /// The local endpoint of the tunnel.
    #[zbus(property)]
    fn local(&self) -> zbus::Result<String>;

    /// The remote endpoint of the tunnel.
    #[zbus(property)]
    fn remote(&self) -> zbus::Result<String>;

    /// The TTL assigned to tunneled packets.
    #[zbus(property)]
    fn ttl(&self) -> zbus::Result<u8>;

    /// The type of service assigned to tunneled packets.
    #[zbus(property)]
    fn tos(&self) -> zbus::Result<u8>;

    /// Whether path MTU discovery is enabled on this tunnel.
    #[zbus(property)]
    fn path_mtu_discovery(&self) -> zbus::Result<bool>;

    /// The key used for tunnel input packets.
    #[zbus(property)]
    fn input_key(&self) -> zbus::Result<String>;

    /// The key used for tunnel output packets.
    #[zbus(property)]
    fn output_key(&self) -> zbus::Result<String>;

    /// The flow label to assign to tunnel packets.
    #[zbus(property)]
    fn flow_label(&self) -> zbus::Result<u32>;

    /// Tunnel flags.
    #[zbus(property)]
    fn flags(&self) -> zbus::Result<u32>;

    /// How many additional levels of encapsulation are permitted.
    #[zbus(property)]
    fn encapsulation_limit(&self) -> zbus::Result<u8>;
}
