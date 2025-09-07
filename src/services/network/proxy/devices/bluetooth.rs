//! NetworkManager Bluetooth Device interface.

use zbus::proxy;

/// Bluetooth Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Bluetooth"
)]
pub(crate) trait DeviceBluetooth {
    /// Bluetooth hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Bluetooth name of the device.
    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;

    /// Bluetooth device capabilities.
    #[zbus(property)]
    fn bt_capabilities(&self) -> zbus::Result<u32>;
}
