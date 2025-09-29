//! NetworkManager Wi-Fi P2P Peer interface.

use zbus::proxy;

/// Wi-Fi P2P Peer.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.WifiP2PPeer"
)]
pub(crate) trait WifiP2PPeer {
    /// The name of the peer.
    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;

    /// The flags of the peer.
    #[zbus(property)]
    fn flags(&self) -> zbus::Result<u32>;

    /// The Wi-Fi Display Information Elements of the peer.
    #[zbus(property)]
    fn wfd_ies(&self) -> zbus::Result<Vec<u8>>;

    /// The hardware address of the peer.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// The current signal strength of the peer.
    #[zbus(property)]
    fn strength(&self) -> zbus::Result<u8>;

    /// The timestamp for the last time the peer was found.
    #[zbus(property)]
    fn last_seen(&self) -> zbus::Result<i32>;
}
