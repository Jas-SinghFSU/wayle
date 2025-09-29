use std::collections::HashMap;

use tokio_util::sync::CancellationToken;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::services::network::types::states::{NMDeviceState, NMDeviceStateReason};

pub(crate) struct DeviceParams<'a> {
    pub connection: &'a Connection,
    pub object_path: OwnedObjectPath,
}

pub(crate) struct LiveDeviceParams<'a> {
    pub connection: &'a Connection,
    pub object_path: OwnedObjectPath,
    pub cancellation_token: &'a CancellationToken,
}

/// Fetched device properties from D-Bus
pub(crate) struct DeviceProperties {
    pub udi: String,
    pub udev_path: String,
    pub interface: String,
    pub ip_interface: String,
    pub driver: String,
    pub driver_version: String,
    pub firmware_version: String,
    pub capabilities: u32,
    pub state: u32,
    pub state_reason: (u32, u32),
    pub active_connection: OwnedObjectPath,
    pub ip4_config: OwnedObjectPath,
    pub dhcp4_config: OwnedObjectPath,
    pub ip6_config: OwnedObjectPath,
    pub dhcp6_config: OwnedObjectPath,
    pub managed: bool,
    pub autoconnect: bool,
    pub firmware_missing: bool,
    pub nm_plugin_missing: bool,
    pub device_type: u32,
    pub available_connections: Vec<OwnedObjectPath>,
    pub physical_port_id: String,
    pub mtu: u32,
    pub metered: u32,
    pub real: bool,
    pub ip4_connectivity: u32,
    pub ip6_connectivity: u32,
    pub interface_flags: u32,
    pub hw_address: String,
    pub ports: Vec<OwnedObjectPath>,
}

/// Connection configuration currently applied to a network device.
///
/// Contains the settings that are actively being used by the device,
/// which may differ from the saved connection profile if it was
/// modified after activation or changed via Reapply.
#[derive(Debug, Clone)]
pub struct AppliedConnection {
    /// Connection settings organized by group (e.g., "ipv4", "connection", "802-11-wireless").
    ///
    /// Each group contains its configuration parameters as key-value pairs.
    /// This is kept as raw data due to the complexity and variety of NetworkManager
    /// connection types (Ethernet, WiFi, VPN, Bridge, etc.), each with different settings.
    pub settings: HashMap<String, HashMap<String, OwnedValue>>,

    /// Version identifier for this applied connection.
    ///
    /// Used to detect concurrent modifications when calling Reapply.
    pub version_id: u64,
}

impl AppliedConnection {
    /// Gets the connection UUID if present.
    pub fn uuid(&self) -> Option<String> {
        self.settings
            .get("connection")
            .and_then(|conn| conn.get("uuid"))
            .and_then(|v| String::try_from(v.clone()).ok())
    }

    /// Gets the connection ID (human-readable name) if present.
    pub fn id(&self) -> Option<String> {
        self.settings
            .get("connection")
            .and_then(|conn| conn.get("id"))
            .and_then(|v| String::try_from(v.clone()).ok())
    }

    /// Gets the connection type (e.g., "802-3-ethernet", "802-11-wireless").
    pub fn connection_type(&self) -> Option<String> {
        self.settings
            .get("connection")
            .and_then(|conn| conn.get("type"))
            .and_then(|v| String::try_from(v.clone()).ok())
    }
}

impl From<(HashMap<String, HashMap<String, OwnedValue>>, u64)> for AppliedConnection {
    fn from((settings, version_id): (HashMap<String, HashMap<String, OwnedValue>>, u64)) -> Self {
        Self {
            settings,
            version_id,
        }
    }
}

/// Event emitted when a device's state changes.
pub struct DeviceStateChangedEvent {
    /// The new device state.
    pub new_state: NMDeviceState,
    /// The old device state.
    pub old_state: NMDeviceState,
    /// The reason for the state change.
    pub reason: NMDeviceStateReason,
}
