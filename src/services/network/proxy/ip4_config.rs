//! NetworkManager IPv4 Configuration interface.

use std::collections::HashMap;

use zbus::{proxy, zvariant::OwnedValue};

/// IPv4 Configuration Set.
///
/// Contains IPv4 configuration information.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.IP4Config"
)]
pub trait IP4Config {
    /// Array of arrays of IPv4 address/prefix/gateway.
    #[zbus(property)]
    fn addresses(&self) -> zbus::Result<Vec<Vec<u32>>>;

    /// Array of IP address data objects.
    #[zbus(property)]
    fn address_data(&self) -> zbus::Result<Vec<HashMap<String, OwnedValue>>>;

    /// The gateway in use.
    #[zbus(property)]
    fn gateway(&self) -> zbus::Result<String>;

    /// Arrays of IPv4 route/prefix/next-hop/metric.
    #[zbus(property)]
    fn routes(&self) -> zbus::Result<Vec<Vec<u32>>>;

    /// Array of IP route data objects.
    #[zbus(property)]
    fn route_data(&self) -> zbus::Result<Vec<HashMap<String, OwnedValue>>>;

    /// Array of nameserver data objects.
    #[zbus(property)]
    fn nameserver_data(&self) -> zbus::Result<Vec<HashMap<String, OwnedValue>>>;

    /// A list of domains this address belongs to.
    #[zbus(property)]
    fn domains(&self) -> zbus::Result<Vec<String>>;

    /// A list of dns searches.
    #[zbus(property)]
    fn searches(&self) -> zbus::Result<Vec<String>>;

    /// A list of DNS options that modify the behavior of the DNS resolver.
    #[zbus(property)]
    fn dns_options(&self) -> zbus::Result<Vec<String>>;

    /// Relative priority of DNS servers.
    #[zbus(property)]
    fn dns_priority(&self) -> zbus::Result<i32>;

    /// Array of Windows Internet Name Service server IP addresses.
    #[zbus(property)]
    fn wins_server_data(&self) -> zbus::Result<Vec<String>>;
}
