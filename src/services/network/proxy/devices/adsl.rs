//! NetworkManager ADSL Device interface.

use zbus::proxy;

/// ADSL Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Adsl"
)]
pub(crate) trait DeviceAdsl {
    /// Indicates whether the physical carrier is found.
    #[zbus(property)]
    fn carrier(&self) -> zbus::Result<bool>;
}
