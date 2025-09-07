//! NetworkManager DHCP4 Configuration interface.

use std::collections::HashMap;

use zbus::{proxy, zvariant::Value as Variant};

/// IPv4 DHCP Client State.
///
/// Contains DHCP configuration received from server.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.DHCP4Config"
)]
pub(crate) trait DHCP4Config {
    /// Configuration options returned by a DHCP server.
    #[zbus(property)]
    fn options(&self) -> zbus::Result<HashMap<String, Variant>>;
}
