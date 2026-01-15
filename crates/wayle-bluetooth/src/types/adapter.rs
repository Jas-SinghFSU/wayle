use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

use zbus::zvariant::Value;

/// Bluetooth address type for adapters and devices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    /// Public Bluetooth address
    Public,
    /// Random Bluetooth address (LE)
    Random,
}

impl Display for AddressType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::Random => write!(f, "random"),
        }
    }
}

impl From<&str> for AddressType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "random" => Self::Random,
            _ => Self::Public,
        }
    }
}

/// Power state of a Bluetooth adapter.
///
/// [experimental]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    /// Adapter is powered on
    On,
    /// Adapter is powered off
    Off,
    /// Adapter is transitioning from off to on
    OffToOn,
    /// Adapter is transitioning from on to off
    OnToOff,
}

impl From<&str> for PowerState {
    fn from(s: &str) -> Self {
        match s {
            "on" => Self::On,
            "off" => Self::Off,
            "off-enabling" => Self::OffToOn,
            "on-disabling" => Self::OnToOff,
            _ => Self::Off,
        }
    }
}

impl Display for PowerState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
            Self::OffToOn => write!(f, "off-enabling"),
            Self::OnToOff => write!(f, "on-disabling"),
        }
    }
}

/// Role capabilities of a Bluetooth adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterRole {
    /// Supports the central role
    Central,
    /// Supports the peripheral role
    Peripheral,
    /// Supports both central and peripheral roles concurrently
    CentralPeripheral,
}

impl From<&str> for AdapterRole {
    fn from(s: &str) -> Self {
        match s {
            "central" => Self::Central,
            "peripheral" => Self::Peripheral,
            "central-peripheral" => Self::CentralPeripheral,
            _ => Self::Central,
        }
    }
}

impl Display for AdapterRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Central => write!(f, "central"),
            Self::Peripheral => write!(f, "peripheral"),
            Self::CentralPeripheral => write!(f, "central-peripheral"),
        }
    }
}

/// Discovery transport filter for Bluetooth scanning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiscoveryTransport {
    /// Interleaved scan, use LE, BREDR, or both depending on what's currently enabled
    #[default]
    Auto,
    /// BR/EDR inquiry only
    BrEdr,
    /// LE scan only
    Le,
}

impl From<&str> for DiscoveryTransport {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "bredr" => Self::BrEdr,
            "le" => Self::Le,
            _ => Self::Auto,
        }
    }
}

impl Display for DiscoveryTransport {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::BrEdr => write!(f, "bredr"),
            Self::Le => write!(f, "le"),
        }
    }
}

/// Discovery filter parameters for Bluetooth device discovery.
pub type DiscoveryFilter<'a> = HashMap<String, Value<'a>>;

/// Options for creating discovery filters with type safety.
#[derive(Debug, Clone, Default)]
pub struct DiscoveryFilterOptions<'a> {
    /// UUIDs to filter for. Only devices advertising these UUIDs will be discovered.
    pub uuids: Option<Vec<&'a str>>,
    /// RSSI threshold. Only devices with RSSI >= threshold will be discovered.
    pub rssi: Option<i16>,
    /// Pathloss threshold. Only devices with Pathloss <= threshold will be discovered.
    pub pathloss: Option<u16>,
    /// Transport type to filter for.
    pub transport: Option<DiscoveryTransport>,
    /// Whether to report duplicate advertisement data.
    pub duplicate_data: Option<bool>,
    /// Whether to make this client discoverable.
    pub discoverable: Option<bool>,
}

impl<'a> DiscoveryFilterOptions<'a> {
    /// Creates a new discovery filter options with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts options to a discovery filter HashMap for D-Bus transmission.
    pub fn to_filter(self) -> DiscoveryFilter<'a> {
        let mut filter = HashMap::new();

        if let Some(uuids) = self.uuids {
            let uuid_values: Vec<Value> = uuids.into_iter().map(Value::from).collect();
            filter.insert("UUIDs".to_string(), Value::from(uuid_values));
        }

        if let Some(rssi) = self.rssi {
            filter.insert("RSSI".to_string(), Value::from(rssi));
        }

        if let Some(pathloss) = self.pathloss {
            filter.insert("Pathloss".to_string(), Value::from(pathloss));
        }

        if let Some(transport) = self.transport {
            filter.insert("Transport".to_string(), Value::from(transport.to_string()));
        }

        if let Some(duplicate_data) = self.duplicate_data {
            filter.insert("DuplicateData".to_string(), Value::from(duplicate_data));
        }

        if let Some(discoverable) = self.discoverable {
            filter.insert("Discoverable".to_string(), Value::from(discoverable));
        }

        filter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_type_from_str_parses_random() {
        assert_eq!(AddressType::from("random"), AddressType::Random);
        assert_eq!(AddressType::from("Random"), AddressType::Random);
        assert_eq!(AddressType::from("RANDOM"), AddressType::Random);
    }

    #[test]
    fn address_type_from_str_defaults_to_public() {
        assert_eq!(AddressType::from("public"), AddressType::Public);
        assert_eq!(AddressType::from("unknown"), AddressType::Public);
        assert_eq!(AddressType::from(""), AddressType::Public);
    }

    #[test]
    fn power_state_from_str_handles_all_variants() {
        assert_eq!(PowerState::from("on"), PowerState::On);
        assert_eq!(PowerState::from("off"), PowerState::Off);
        assert_eq!(PowerState::from("off-enabling"), PowerState::OffToOn);
        assert_eq!(PowerState::from("on-disabling"), PowerState::OnToOff);
    }

    #[test]
    fn power_state_from_str_defaults_to_off() {
        assert_eq!(PowerState::from("unknown"), PowerState::Off);
        assert_eq!(PowerState::from(""), PowerState::Off);
    }

    #[test]
    fn adapter_role_from_str_handles_all_variants() {
        assert_eq!(AdapterRole::from("central"), AdapterRole::Central);
        assert_eq!(AdapterRole::from("peripheral"), AdapterRole::Peripheral);
        assert_eq!(
            AdapterRole::from("central-peripheral"),
            AdapterRole::CentralPeripheral
        );
    }

    #[test]
    fn adapter_role_from_str_defaults_to_central() {
        assert_eq!(AdapterRole::from("unknown"), AdapterRole::Central);
        assert_eq!(AdapterRole::from(""), AdapterRole::Central);
    }

    #[test]
    fn discovery_transport_from_str_handles_all_variants() {
        assert_eq!(DiscoveryTransport::from("auto"), DiscoveryTransport::Auto);
        assert_eq!(DiscoveryTransport::from("Auto"), DiscoveryTransport::Auto);
        assert_eq!(DiscoveryTransport::from("bredr"), DiscoveryTransport::BrEdr);
        assert_eq!(DiscoveryTransport::from("BREDR"), DiscoveryTransport::BrEdr);
        assert_eq!(DiscoveryTransport::from("le"), DiscoveryTransport::Le);
        assert_eq!(DiscoveryTransport::from("LE"), DiscoveryTransport::Le);
    }

    #[test]
    fn discovery_transport_from_str_defaults_to_auto() {
        assert_eq!(
            DiscoveryTransport::from("unknown"),
            DiscoveryTransport::Auto
        );
        assert_eq!(DiscoveryTransport::from(""), DiscoveryTransport::Auto);
    }

    #[test]
    fn discovery_filter_to_filter_with_empty_options_returns_empty_map() {
        let options = DiscoveryFilterOptions::new();
        let filter = options.to_filter();

        assert!(filter.is_empty());
    }

    #[test]
    fn discovery_filter_to_filter_with_uuids_includes_uuids_field() {
        let options = DiscoveryFilterOptions {
            uuids: Some(vec!["0000110a-0000-1000-8000-00805f9b34fb"]),
            ..Default::default()
        };
        let filter = options.to_filter();

        assert!(filter.contains_key("UUIDs"));
        assert_eq!(filter.len(), 1);
    }

    #[test]
    fn discovery_filter_to_filter_with_rssi_includes_rssi_field() {
        let options = DiscoveryFilterOptions {
            rssi: Some(-70),
            ..Default::default()
        };
        let filter = options.to_filter();

        assert!(filter.contains_key("RSSI"));
        assert_eq!(filter.len(), 1);
    }

    #[test]
    fn discovery_filter_to_filter_with_all_options_includes_all_fields() {
        let options = DiscoveryFilterOptions {
            uuids: Some(vec!["0000110a-0000-1000-8000-00805f9b34fb"]),
            rssi: Some(-70),
            pathloss: Some(10),
            transport: Some(DiscoveryTransport::Le),
            duplicate_data: Some(true),
            discoverable: Some(false),
        };
        let filter = options.to_filter();

        assert_eq!(filter.len(), 6);
        assert!(filter.contains_key("UUIDs"));
        assert!(filter.contains_key("RSSI"));
        assert!(filter.contains_key("Pathloss"));
        assert!(filter.contains_key("Transport"));
        assert!(filter.contains_key("DuplicateData"));
        assert!(filter.contains_key("Discoverable"));
    }
}
