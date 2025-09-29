//! NetworkManager Bond Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

/// Bond Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Bond"
)]
pub(crate) trait DeviceBond {
    /// Hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Indicates whether the bond has any slave devices with carrier.
    #[zbus(property)]
    fn carrier(&self) -> zbus::Result<bool>;

    /// Array of object paths representing slave devices which are part of this bond.
    #[zbus(property)]
    fn slaves(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}
