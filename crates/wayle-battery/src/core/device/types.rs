use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

/// Parameters for creating a Device instance.
#[doc(hidden)]
pub struct DeviceParams<'a> {
    /// D-Bus connection.
    pub connection: &'a Connection,
    /// Device path.
    pub device_path: &'a OwnedObjectPath,
}

/// Parameters for creating a live Device instance.
#[doc(hidden)]
pub struct LiveDeviceParams<'a> {
    /// D-Bus connection.
    pub connection: &'a Connection,
    /// Device path.
    pub device_path: &'a OwnedObjectPath,
    /// Cancellation token for monitoring.
    pub cancellation_token: &'a CancellationToken,
}

pub(crate) struct DeviceProps {
    pub native_path: String,
    pub vendor: String,
    pub model: String,
    pub serial: String,
    pub update_time: u64,
    pub device_type: u32,
    pub power_supply: bool,
    pub has_history: bool,
    pub has_statistics: bool,
    pub online: bool,
    pub energy: f64,
    pub energy_empty: f64,
    pub energy_full: f64,
    pub energy_full_design: f64,
    pub energy_rate: f64,
    pub voltage: f64,
    pub charge_cycles: i32,
    pub luminosity: f64,
    pub time_to_empty: i64,
    pub time_to_full: i64,
    pub percentage: f64,
    pub temperature: f64,
    pub is_present: bool,
    pub state: u32,
    pub is_rechargeable: bool,
    pub capacity: f64,
    pub technology: u32,
    pub warning_level: u32,
    pub battery_level: u32,
    pub icon_name: String,
    pub charge_start_threshold: u32,
    pub charge_end_threshold: u32,
    pub charge_threshold_enabled: bool,
    pub charge_threshold_supported: bool,
    pub charge_threshold_settings_supported: u32,
    pub voltage_min_design: f64,
    pub voltage_max_design: f64,
    pub capacity_level: String,
}
