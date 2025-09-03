use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

pub(crate) struct DeviceWifiParams<'a> {
    pub connection: &'a Connection,
    pub device_path: OwnedObjectPath,
}

pub(crate) struct LiveDeviceWifiParams<'a> {
    pub connection: &'a Connection,
    pub device_path: OwnedObjectPath,
    pub cancellation_token: &'a CancellationToken,
}

/// WiFi bitrate in kilobits per second.
pub type BitrateKbps = u32;

/// Boot time in milliseconds.
pub type BootTimeMs = i64;

/// Wireless device capabilities flags.
pub type WirelessCapabilities = u32;

pub(crate) struct WifiProperties {
    pub perm_hw_address: String,
    pub mode: u32,
    pub bitrate: u32,
    pub access_points: Vec<OwnedObjectPath>,
    pub active_access_point: OwnedObjectPath,
    pub wireless_capabilities: u32,
    pub last_scan: i64,
}
