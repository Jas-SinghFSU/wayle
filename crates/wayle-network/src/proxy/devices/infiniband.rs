//! NetworkManager InfiniBand Device interface.

use zbus::proxy;

/// InfiniBand Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Infiniband"
)]
pub(crate) trait DeviceInfiniband {
    /// Hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Indicates whether the physical carrier is found.
    #[zbus(property)]
    fn carrier(&self) -> zbus::Result<bool>;
}
