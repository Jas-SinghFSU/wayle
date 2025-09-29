//! NetworkManager WPAN Device interface.

use zbus::proxy;

/// IEEE 802.15.4 (WPAN) MAC Layer Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Wpan"
)]
pub(crate) trait DeviceWpan {
    /// The active hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;
}
