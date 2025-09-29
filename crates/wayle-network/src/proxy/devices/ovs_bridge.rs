//! NetworkManager OVS Bridge Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

/// OVS Bridge Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.OvsBridge"
)]
pub(crate) trait DeviceOvsBridge {
    /// Array of object paths representing slave devices which are part of this bridge.
    #[zbus(property)]
    fn slaves(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}
