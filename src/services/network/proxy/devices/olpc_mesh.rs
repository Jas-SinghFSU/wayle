//! NetworkManager OLPC Mesh Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

/// OLPC Mesh Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.OlpcMesh"
)]
pub(crate) trait DeviceOlpcMesh {
    /// The hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// The object path of the companion device.
    #[zbus(property)]
    fn companion(&self) -> zbus::Result<OwnedObjectPath>;

    /// The currently active channel.
    #[zbus(property)]
    fn active_channel(&self) -> zbus::Result<u32>;
}
