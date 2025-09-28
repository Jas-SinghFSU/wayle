use std::collections::HashMap;

use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::services::bluetooth::types::ServiceNotification;

/// Event emitted when a device is disconnected.
pub struct DisconnectedEvent {
    /// Reason code for the disconnection.
    pub reason: u8,
    /// Human-readable message describing the disconnection.
    pub message: String,
}

/// Manufacturer-specific advertisement data keyed by company ID.
pub type ManufacturerData = HashMap<u16, Vec<u8>>;
/// Advertisement data keyed by AD type.
pub type AdvertisingData = HashMap<u8, Vec<u8>>;
/// Service-specific advertisement data keyed by UUID.
pub type ServiceData = HashMap<String, Vec<u8>>;
/// Device set membership with object path and properties.
pub type DeviceSet = (OwnedObjectPath, HashMap<String, OwnedValue>);

/// Context for static device operations
pub(crate) struct DeviceParams<'a> {
    /// D-Bus connection for device communication
    pub connection: &'a Connection,
    /// Device object path
    pub path: OwnedObjectPath,
    /// Channel for sending service notifications
    pub notifier_tx: &'a broadcast::Sender<ServiceNotification>,
}

/// Context for live device operations with monitoring
pub(crate) struct LiveDeviceParams<'a> {
    /// D-Bus connection for device communication
    pub connection: &'a Connection,
    /// Device object path
    pub path: OwnedObjectPath,
    /// Token for cancelling monitoring operations
    pub cancellation_token: &'a CancellationToken,
    /// Channel for sending service notifications
    pub notifier_tx: &'a broadcast::Sender<ServiceNotification>,
}

pub(crate) struct DeviceProperties {
    pub address: String,
    pub address_type: String,
    pub name: Option<String>,
    pub battery_percentage: Option<u8>,
    pub icon: Option<String>,
    pub class: Option<u32>,
    pub appearance: Option<u16>,
    pub uuids: Option<Vec<String>>,
    pub paired: bool,
    pub bonded: bool,
    pub connected: bool,
    pub trused: bool,
    pub blocked: bool,
    pub wake_allowed: bool,
    pub alias: String,
    pub adapter: OwnedObjectPath,
    pub legacy_pairing: bool,
    pub cable_pairing: bool,
    pub modalias: Option<String>,
    pub rssi: Option<i16>,
    pub tx_power: Option<i16>,
    pub manufacturer_data: Option<ManufacturerData>,
    pub service_data: Option<ServiceData>,
    pub services_resolved: bool,
    pub advertising_flags: Vec<u8>,
    pub advertising_data: AdvertisingData,
    pub sets: Vec<DeviceSet>,
    pub preferred_bearer: Option<String>,
}
