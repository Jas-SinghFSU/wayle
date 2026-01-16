//! NetworkManager WireGuard Device interface.

use zbus::proxy;

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.WireGuard"
)]
pub(crate) trait DeviceWireGuard {
    /// 32-byte public key used by this interface.
    #[zbus(property)]
    fn public_key(&self) -> zbus::Result<Vec<u8>>;

    /// Local UDP listen port.
    #[zbus(property)]
    fn listen_port(&self) -> zbus::Result<u16>;

    /// Optional firewall mark for outgoing packets.
    #[zbus(property)]
    fn fw_mark(&self) -> zbus::Result<u32>;
}
