use std::collections::HashMap;

use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::types::ServiceNotification;

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
/// Device set membership information.
///
/// Represents a Bluetooth Coordinated Set that this device belongs to.
#[derive(Debug, Clone)]
pub struct DeviceSet {
    /// Object path of the device set.
    pub path: OwnedObjectPath,
    /// The adapter this set belongs to.
    pub adapter: Option<OwnedObjectPath>,
    /// Whether devices in the set should auto-connect together.
    pub auto_connect: bool,
    /// List of device paths that are members of this set.
    pub devices: Vec<OwnedObjectPath>,
    /// Number of members in the set.
    pub size: u8,
}

impl DeviceSet {
    pub(crate) fn from_dbus(path: OwnedObjectPath, props: HashMap<String, OwnedValue>) -> Self {
        let adapter = props
            .get("Adapter")
            .and_then(|v| OwnedObjectPath::try_from(v.clone()).ok());

        let auto_connect = props
            .get("AutoConnect")
            .and_then(|v| bool::try_from(v).ok())
            .unwrap_or(false);

        let devices = props
            .get("Devices")
            .and_then(|v| Vec::<OwnedObjectPath>::try_from(v.clone()).ok())
            .unwrap_or_default();

        let size = props
            .get("Size")
            .and_then(|v| u8::try_from(v).ok())
            .unwrap_or(0);

        Self {
            path,
            adapter,
            auto_connect,
            devices,
            size,
        }
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
