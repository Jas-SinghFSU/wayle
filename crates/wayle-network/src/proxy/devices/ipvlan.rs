//! NetworkManager IPVLAN Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Ipvlan"
)]
pub(crate) trait DeviceIpvlan {
    /// The object path of the parent device.
    #[zbus(property)]
    fn parent(&self) -> zbus::Result<OwnedObjectPath>;

    /// Hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// The ipvlan mode.
    #[zbus(property)]
    fn mode(&self) -> zbus::Result<u32>;

    /// Whether the device is blocked from communicating with all other devices on the same physical port.
    #[zbus(property)]
    fn private(&self) -> zbus::Result<bool>;

    /// Whether the device receives and responds to external DHCP and ARP requests.
    #[zbus(property)]
    fn vepa(&self) -> zbus::Result<bool>;
}
