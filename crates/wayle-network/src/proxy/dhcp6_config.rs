//! NetworkManager DHCP6 Configuration interface.

use std::collections::HashMap;

use zbus::{proxy, zvariant::Value as Variant};

/// IPv6 DHCP Client State.
///
/// Contains DHCPv6 configuration received from server.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.DHCP6Config"
)]
pub(crate) trait DHCP6Config {
    /// Configuration options returned by a DHCP server.
    #[zbus(property)]
    fn options(&self) -> zbus::Result<HashMap<String, Variant>>;
}
