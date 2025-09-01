use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

pub(crate) struct DeviceWiredParams<'a> {
    pub connection: &'a Connection,
    pub device_path: OwnedObjectPath,
}

pub(crate) struct LiveDeviceWiredParams<'a> {
    pub connection: &'a Connection,
    pub device_path: OwnedObjectPath,
    pub cancellation_token: CancellationToken,
}

pub type SpeedMbps = u32;

pub(crate) struct WiredProperties {
    pub perm_hw_address: String,
    pub speed: u32,
    pub s390_subchannels: Vec<String>,
}
