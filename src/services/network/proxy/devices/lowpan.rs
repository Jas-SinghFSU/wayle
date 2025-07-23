//! NetworkManager 6LoWPAN Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

/// 6LoWPAN Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Lowpan"
)]
pub trait DeviceLowpan {
    /// Hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// The object path of the parent device.
    #[zbus(property)]
    fn parent(&self) -> zbus::Result<OwnedObjectPath>;
}

