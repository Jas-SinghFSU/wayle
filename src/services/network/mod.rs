/// Core domain models for NetworkManager objects
pub mod core;
/// Base-line discovery for the network service
mod discovery;
/// Network service errors
mod error;
/// Base-line monitoring for the network service
mod monitoring;
/// D-Bus proxy implementations for NetworkManager interfaces.
mod proxy;
/// High-level service API for network operations.
mod service;
/// Type definitions for NetworkManager enums, flags, and states.
mod types;
/// Wi-Fi service API
mod wifi;
/// Wired service API
mod wired;

pub use core::access_point::{AccessPoint, BSSID, NetworkIdentifier, SSID, SecurityType};

pub use error::NetworkError;
pub use proxy::*;
pub use service::NetworkService;
pub use types::*;
pub use wifi::Wifi;
pub use wired::Wired;
