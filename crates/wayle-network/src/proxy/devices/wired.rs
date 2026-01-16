//! NetworkManager Wired Device interface.

use zbus::proxy;

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Wired"
)]
pub(crate) trait DeviceWired {
    /// Active hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Permanent hardware address of the device.
    #[zbus(property)]
    fn perm_hw_address(&self) -> zbus::Result<String>;

    /// Design speed of the device, in megabits/second (Mb/s).
    #[zbus(property)]
    fn speed(&self) -> zbus::Result<u32>;

    /// Array of S/390 subchannels for S/390 or z/Architecture devices.
    #[zbus(property)]
    fn s390_subchannels(&self) -> zbus::Result<Vec<String>>;

    /// Indicates whether the physical carrier is found.
    #[zbus(property)]
    fn carrier(&self) -> zbus::Result<bool>;
}
