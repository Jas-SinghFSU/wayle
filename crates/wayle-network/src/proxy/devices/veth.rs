//! NetworkManager Virtual Ethernet Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

/// Virtual Ethernet Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Veth"
)]
pub(crate) trait DeviceVeth {
    /// Object path of the peer device of this Veth.
    #[zbus(property)]
    fn peer(&self) -> zbus::Result<OwnedObjectPath>;
}
