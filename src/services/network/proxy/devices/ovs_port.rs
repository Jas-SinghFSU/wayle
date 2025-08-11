//! NetworkManager OVS Port Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

/// OVS Port Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.OvsPort"
)]
pub trait DeviceOvsPort {
    /// Array of object paths representing slave devices which are part of this port.
    #[zbus(property)]
    fn slaves(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}
