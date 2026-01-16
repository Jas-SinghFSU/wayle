//! NetworkManager PPP interface.

use std::collections::HashMap;

use zbus::{proxy, zvariant::OwnedValue};

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.PPP"
)]
pub(crate) trait PPP {
    /// Asks NetworkManager for PPP secrets.
    ///
    /// # Returns
    /// * (username, password)
    fn need_secrets(&self) -> zbus::Result<(String, String)>;

    /// Set IPv4 configuration for the PPP interface.
    ///
    /// # Arguments
    /// * `config` - Dictionary with keys: address, prefix, gateway
    fn set_ip4_config(&self, config: HashMap<String, OwnedValue>) -> zbus::Result<()>;

    /// Set IPv6 configuration for the PPP interface.
    ///
    /// # Arguments
    /// * `config` - Dictionary with keys: address, prefix, gateway
    fn set_ip6_config(&self, config: HashMap<String, OwnedValue>) -> zbus::Result<()>;

    /// Set PPP connection state.
    ///
    /// # Arguments
    /// * `state` - NMPPPState
    fn set_state(&self, state: u32) -> zbus::Result<()>;

    /// Set the ifindex of the PPP interface.
    ///
    /// # Arguments
    /// * `ifindex` - Interface index
    fn set_ifindex(&self, ifindex: i32) -> zbus::Result<()>;
}
