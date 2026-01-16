//! NetworkManager Modem Device interface.

use zbus::proxy;

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Modem"
)]
pub(crate) trait DeviceModem {
    /// The generic family of access technologies the modem supports.
    #[zbus(property)]
    fn modem_capabilities(&self) -> zbus::Result<u32>;

    /// The generic family of access technologies the modem currently supports.
    #[zbus(property)]
    fn current_capabilities(&self) -> zbus::Result<u32>;

    /// An identifier used by the modem backend that aims to uniquely identify the device.
    #[zbus(property)]
    fn device_id(&self) -> zbus::Result<String>;

    /// The MCC and MNC (concatenated) of the network the modem is connected to.
    #[zbus(property)]
    fn operator_code(&self) -> zbus::Result<String>;

    /// The Access Point Name the modem is connected to.
    #[zbus(property)]
    fn apn(&self) -> zbus::Result<String>;
}
