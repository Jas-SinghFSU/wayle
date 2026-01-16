use std::collections::HashMap;

use tokio_util::sync::CancellationToken;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::types::states::{NMDeviceState, NMDeviceStateReason};

#[doc(hidden)]
pub struct DeviceParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) object_path: OwnedObjectPath,
}

#[doc(hidden)]
pub struct LiveDeviceParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) object_path: OwnedObjectPath,
    pub(crate) cancellation_token: &'a CancellationToken,
}

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

#[cfg(test)]
mod tests {
    use zbus::zvariant::Value;

    use super::*;

    fn create_test_settings() -> HashMap<String, HashMap<String, OwnedValue>> {
        let mut settings = HashMap::new();
        let mut connection_section = HashMap::new();
        connection_section.insert(
            "uuid".to_string(),
            Value::from("test-uuid-123").try_to_owned().unwrap(),
        );
        connection_section.insert(
            "id".to_string(),
            Value::from("Test Connection").try_to_owned().unwrap(),
        );
        connection_section.insert(
            "type".to_string(),
            Value::from("802-11-wireless").try_to_owned().unwrap(),
        );
        settings.insert("connection".to_string(), connection_section);
        settings
    }

    #[test]
    fn uuid_returns_some_when_connection_uuid_exists() {
        let settings = create_test_settings();
        let applied = AppliedConnection {
            settings,
            version_id: 1,
        };

        assert_eq!(applied.uuid(), Some("test-uuid-123".to_string()));
    }

    #[test]
    fn uuid_returns_none_when_connection_section_missing() {
        let settings = HashMap::new();
        let applied = AppliedConnection {
            settings,
            version_id: 1,
        };

        assert_eq!(applied.uuid(), None);
    }

    #[test]
    fn uuid_returns_none_when_uuid_field_missing() {
        let mut settings = HashMap::new();
        let mut connection_section = HashMap::new();
        connection_section.insert(
            "id".to_string(),
            Value::from("Test").try_to_owned().unwrap(),
        );
        settings.insert("connection".to_string(), connection_section);

        let applied = AppliedConnection {
            settings,
            version_id: 1,
        };

        assert_eq!(applied.uuid(), None);
    }

    #[test]
    fn uuid_returns_none_when_uuid_not_string() {
        let mut settings = HashMap::new();
        let mut connection_section = HashMap::new();
        connection_section.insert(
            "uuid".to_string(),
            Value::from(42u32).try_to_owned().unwrap(),
        );
        settings.insert("connection".to_string(), connection_section);

        let applied = AppliedConnection {
            settings,
            version_id: 1,
        };

        assert_eq!(applied.uuid(), None);
    }

    #[test]
    fn id_returns_some_when_connection_id_exists() {
        let settings = create_test_settings();
        let applied = AppliedConnection {
            settings,
            version_id: 1,
        };

        assert_eq!(applied.id(), Some("Test Connection".to_string()));
    }

    #[test]
    fn id_returns_none_when_connection_section_missing() {
        let settings = HashMap::new();
        let applied = AppliedConnection {
            settings,
            version_id: 1,
        };

        assert_eq!(applied.id(), None);
    }

    #[test]
    fn id_returns_none_when_id_field_missing() {
        let mut settings = HashMap::new();
        let mut connection_section = HashMap::new();
        connection_section.insert(
            "uuid".to_string(),
            Value::from("test-uuid").try_to_owned().unwrap(),
        );
        settings.insert("connection".to_string(), connection_section);

        let applied = AppliedConnection {
            settings,
            version_id: 1,
        };

        assert_eq!(applied.id(), None);
    }

    #[test]
    fn id_returns_none_when_id_not_string() {
        let mut settings = HashMap::new();
        let mut connection_section = HashMap::new();
        connection_section.insert("id".to_string(), Value::from(true).try_to_owned().unwrap());
        settings.insert("connection".to_string(), connection_section);

        let applied = AppliedConnection {
            settings,
            version_id: 1,
        };

        assert_eq!(applied.id(), None);
    }

    #[test]
    fn connection_type_returns_some_when_type_exists() {
        let settings = create_test_settings();
        let applied = AppliedConnection {
            settings,
            version_id: 1,
        };

        assert_eq!(
            applied.connection_type(),
            Some("802-11-wireless".to_string())
        );
    }

    #[test]
    fn connection_type_returns_none_when_connection_section_missing() {
        let settings = HashMap::new();
        let applied = AppliedConnection {
            settings,
            version_id: 1,
        };

        assert_eq!(applied.connection_type(), None);
    }

    #[test]
    fn connection_type_returns_none_when_type_field_missing() {
        let mut settings = HashMap::new();
        let mut connection_section = HashMap::new();
        connection_section.insert(
            "uuid".to_string(),
            Value::from("test-uuid").try_to_owned().unwrap(),
        );
        settings.insert("connection".to_string(), connection_section);

        let applied = AppliedConnection {
            settings,
            version_id: 1,
        };

        assert_eq!(applied.connection_type(), None);
    }

    #[test]
    fn connection_type_returns_none_when_type_not_string() {
        let mut settings = HashMap::new();
        let mut connection_section = HashMap::new();
        connection_section.insert(
            "type".to_string(),
            Value::from(123i32).try_to_owned().unwrap(),
        );
        settings.insert("connection".to_string(), connection_section);

        let applied = AppliedConnection {
            settings,
            version_id: 1,
        };

        assert_eq!(applied.connection_type(), None);
    }
}
