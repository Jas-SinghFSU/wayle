//! NetworkManager VPN Connection interface.

use zbus::proxy;

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.VPN.Connection"
)]
pub(crate) trait VPNConnection {
    /// The VPN-specific state of the connection.
    #[zbus(property)]
    fn vpn_state(&self) -> zbus::Result<u32>;

    /// The banner string of the VPN connection.
    #[zbus(property)]
    fn banner(&self) -> zbus::Result<String>;

    /// Emitted when the state of the VPN connection has changed.
    #[zbus(signal, name = "VpnStateChanged")]
    fn vpn_connection_state_changed(&self, state: u32, reason: u32) -> zbus::Result<()>;
}
