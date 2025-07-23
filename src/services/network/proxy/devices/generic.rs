//! NetworkManager Generic Device interface.

use zbus::proxy;

/// Generic Network Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Generic"
)]
pub trait DeviceGeneric {
    /// Hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// A descriptive name for the device type.
    #[zbus(property)]
    fn type_description(&self) -> zbus::Result<String>;
}

