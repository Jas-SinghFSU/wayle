//! NetworkManager Wireless Device interface.

use std::collections::HashMap;

use zbus::{
    proxy,
    zvariant::{OwnedObjectPath, OwnedValue},
};

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Wireless"
)]
pub(crate) trait DeviceWireless {
    /// Get the list of all access points visible to this device, including hidden ones.
    fn get_all_access_points(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// Request the device to scan for available access points.
    ///
    /// # Arguments
    /// * `options` - Options for scanning (currently unused)
    fn request_scan(&self, options: HashMap<String, OwnedValue>) -> zbus::Result<()>;

    /// The active hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// The permanent hardware address of the device.
    #[zbus(property)]
    fn perm_hw_address(&self) -> zbus::Result<String>;

    /// The operating mode of the wireless device.
    #[zbus(property)]
    fn mode(&self) -> zbus::Result<u32>;

    /// The bit rate currently used by the wireless device, in kilobits/second (Kb/s).
    #[zbus(property)]
    fn bitrate(&self) -> zbus::Result<u32>;

    /// List of object paths of access points visible to this wireless device.
    #[zbus(property)]
    fn access_points(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// Object path of the access point currently used by the wireless device.
    #[zbus(property)]
    fn active_access_point(&self) -> zbus::Result<OwnedObjectPath>;

    /// The capabilities of the wireless device.
    #[zbus(property)]
    fn wireless_capabilities(&self) -> zbus::Result<u32>;

    /// The timestamp (in CLOCK_BOOTTIME milliseconds) for the last finished network scan.
    #[zbus(property)]
    fn last_scan(&self) -> zbus::Result<i64>;

    /// Emitted when a new access point is found by the device.
    #[zbus(signal)]
    fn access_point_added(&self, access_point: OwnedObjectPath) -> zbus::Result<()>;

    /// Emitted when an access point disappears from view of the device.
    #[zbus(signal)]
    fn access_point_removed(&self, access_point: OwnedObjectPath) -> zbus::Result<()>;
}
