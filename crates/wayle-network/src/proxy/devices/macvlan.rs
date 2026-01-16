//! NetworkManager MACVLAN Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Macvlan"
)]
pub(crate) trait DeviceMacvlan {
    /// The object path of the parent device.
    #[zbus(property)]
    fn parent(&self) -> zbus::Result<OwnedObjectPath>;

    /// Hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// The macvlan mode.
    #[zbus(property)]
    fn mode(&self) -> zbus::Result<String>;

    /// Whether the device is blocked from communicating with all other devices on the same physical port.
    #[zbus(property)]
    fn no_promisc(&self) -> zbus::Result<bool>;

    /// Whether the device is a macvtap.
    #[zbus(property)]
    fn tap(&self) -> zbus::Result<bool>;
}
