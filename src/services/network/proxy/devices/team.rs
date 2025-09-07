//! NetworkManager Team Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

/// Team Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Team"
)]
pub(crate) trait DeviceTeam {
    /// Hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Indicates whether the team device has carrier.
    #[zbus(property)]
    fn carrier(&self) -> zbus::Result<bool>;

    /// Array of object paths representing slave devices which are part of this team.
    #[zbus(property)]
    fn slaves(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// The JSON configuration currently applied on the device.
    #[zbus(property)]
    fn config(&self) -> zbus::Result<String>;
}
