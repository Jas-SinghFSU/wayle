//! NetworkManager HSR Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Hsr"
)]
pub(crate) trait DeviceHsr {
    /// The path of the port1 device.
    #[zbus(property)]
    fn port1(&self) -> zbus::Result<OwnedObjectPath>;

    /// The path of the port2 device.
    #[zbus(property)]
    fn port2(&self) -> zbus::Result<OwnedObjectPath>;

    /// The last supervision frame's multicast address for frames received on the ring.
    #[zbus(property)]
    fn supervision_address(&self) -> zbus::Result<String>;

    /// The PRP variant of the HSR protocol.
    #[zbus(property)]
    fn prp(&self) -> zbus::Result<bool>;

    /// Supervision frame multicast address.
    #[zbus(property)]
    fn multicast_spec(&self) -> zbus::Result<u8>;

    /// The protocol used for the ring.
    #[zbus(property)]
    fn protocol(&self) -> zbus::Result<String>;
}
