use std::collections::HashMap;

use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::types::{ServiceNotification, device::DisconnectReason};

/// Event emitted when a device is disconnected.
pub struct DisconnectedEvent {
    /// Reason for the disconnection.
    pub reason: DisconnectReason,
    /// Human-readable message from BlueZ.
    pub message: String,
}

/// Manufacturer-specific advertisement data keyed by company ID.
pub type ManufacturerData = HashMap<u16, Vec<u8>>;
/// Advertisement data keyed by AD type.
pub type AdvertisingData = HashMap<u8, Vec<u8>>;
/// Service-specific advertisement data keyed by UUID.
pub type ServiceData = HashMap<String, Vec<u8>>;
/// Device set membership from the `Device.Sets` property.
///
/// For full set properties (Adapter, Devices, Size), query
/// `org.bluez.DeviceSet1` at `path`.
#[derive(Debug, Clone)]
pub struct DeviceSet {
    /// Object path of the device set.
    pub path: OwnedObjectPath,
    /// Rank of this device within the set.
    pub rank: Option<u8>,
}

impl DeviceSet {
    pub(crate) fn from_dbus(path: OwnedObjectPath, props: HashMap<String, OwnedValue>) -> Self {
        let rank = props.get("Rank").and_then(|value| u8::try_from(value).ok());

        Self { path, rank }
    }
}

#[doc(hidden)]
pub struct DeviceParams<'a> {
    pub connection: &'a Connection,
    pub path: OwnedObjectPath,
    pub(crate) notifier_tx: &'a broadcast::Sender<ServiceNotification>,
}

#[doc(hidden)]
pub struct LiveDeviceParams<'a> {
    pub connection: &'a Connection,
    pub path: OwnedObjectPath,
    pub cancellation_token: &'a CancellationToken,
    pub(crate) notifier_tx: &'a broadcast::Sender<ServiceNotification>,
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
    pub trusted: bool,
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
