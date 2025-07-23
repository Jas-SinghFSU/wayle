//! NetworkManager Dummy Device interface.

use zbus::proxy;

/// Dummy Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Dummy"
)]
pub trait DeviceDummy {
    /// Hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;
}

