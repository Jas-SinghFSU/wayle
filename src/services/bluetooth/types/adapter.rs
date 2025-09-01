use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

use serde::{Deserialize, Serialize};
use zbus::zvariant::Value;

/// Bluetooth address type for adapters and devices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiscoveryTransport {
    /// Interleaved scan, use LE, BREDR, or both depending on what's currently enabled
    Auto,
    /// BR/EDR inquiry only
    BrEdr,
    /// LE scan only
    Le,
}

impl Default for DiscoveryTransport {
    fn default() -> Self {
        Self::Auto
    }
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
