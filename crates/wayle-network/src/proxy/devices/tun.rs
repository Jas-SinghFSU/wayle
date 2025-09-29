//! NetworkManager TUN/TAP Device interface.

use zbus::proxy;

/// TUN/TAP Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Tun"
)]
pub(crate) trait DeviceTun {
    /// Hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// The operating mode of the virtual device.
    #[zbus(property)]
    fn mode(&self) -> zbus::Result<String>;

    /// The user that will own the device.
    #[zbus(property)]
    fn owner(&self) -> zbus::Result<i64>;

    /// The group that will own the device.
    #[zbus(property)]
    fn group(&self) -> zbus::Result<i64>;

    /// If the device has the IFF_NO_PI flag.
    #[zbus(property)]
    fn no_pi(&self) -> zbus::Result<bool>;

    /// If the device has the IFF_VNET_HDR flag.
    #[zbus(property)]
    fn vnet_hdr(&self) -> zbus::Result<bool>;

    /// If the device has the IFF_MULTI_QUEUE flag.
    #[zbus(property)]
    fn multi_queue(&self) -> zbus::Result<bool>;
}
