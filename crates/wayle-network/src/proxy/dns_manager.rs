//! NetworkManager DNS Manager interface.

use std::collections::HashMap;

use zbus::{proxy, zvariant::OwnedValue};

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.DnsManager",
    default_path = "/org/freedesktop/NetworkManager/DnsManager"
)]
pub(crate) trait DnsManager {
    /// The current DNS processing mode.
    #[zbus(property)]
    fn mode(&self) -> zbus::Result<String>;

    /// The current resolv.conf management mode.
    #[zbus(property)]
    fn rc_manager(&self) -> zbus::Result<String>;

    /// The current DNS configuration.
    #[zbus(property)]
    fn configuration(&self) -> zbus::Result<Vec<HashMap<String, OwnedValue>>>;
}
