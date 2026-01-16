use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

#[doc(hidden)]
pub struct DeviceWiredParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) device_path: OwnedObjectPath,
}

#[doc(hidden)]
pub struct LiveDeviceWiredParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) device_path: OwnedObjectPath,
    pub(crate) cancellation_token: &'a CancellationToken,
}

/// Network speed in megabits per second.
pub type SpeedMbps = u32;

pub(crate) struct WiredProperties {
    pub perm_hw_address: String,
    pub speed: u32,
    pub s390_subchannels: Vec<String>,
}
