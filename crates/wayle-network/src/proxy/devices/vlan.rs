//! NetworkManager VLAN Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Vlan"
)]
pub(crate) trait DeviceVlan {
    /// Hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Indicates whether the physical carrier is found.
    #[zbus(property)]
    fn carrier(&self) -> zbus::Result<bool>;

    /// Object path of the parent device of this VLAN device.
    #[zbus(property)]
    fn parent(&self) -> zbus::Result<OwnedObjectPath>;

    /// The VLAN ID of this VLAN interface.
    #[zbus(property)]
    fn vlan_id(&self) -> zbus::Result<u32>;
}
