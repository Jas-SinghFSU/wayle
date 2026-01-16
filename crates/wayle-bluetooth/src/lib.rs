//! Bluetooth device management via BlueZ D-Bus.
//!
//! # Reactive Properties
//!
//! All state is exposed through [`Property<T>`](wayle_common::Property) fields:
//!
//! - **`.get()`** - Returns a point-in-time snapshot of the value
//! - **`.watch()`** - Returns a stream that yields the current value immediately,
//!   then emits on every change
//!
//! # Live vs Snapshot Instances
//!
//! Service fields (`adapters`, `devices`, `primary_adapter`) contain **live** instances
//! that automatically update when BlueZ properties change.
//!
//! Explicit lookups via [`device()`](crate::BluetoothService::device) and
//! [`adapter()`](crate::BluetoothService::adapter) return **snapshots** - the values
//! are frozen at query time.
//!
//! For monitored instances that track changes, use
//! [`device_monitored()`](crate::BluetoothService::device_monitored) or
//! [`adapter_monitored()`](crate::BluetoothService::adapter_monitored).
//!
//! # Service Fields
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `adapters` | `Vec<Arc<Adapter>>` | All Bluetooth adapters on the system |
//! | `primary_adapter` | `Option<Arc<Adapter>>` | Active adapter for operations |
//! | `devices` | `Vec<Arc<Device>>` | All discovered devices |
//! | `available` | `bool` | Whether any adapter is present |
//! | `enabled` | `bool` | Whether any adapter is powered |
//! | `connected` | `Vec<String>` | Addresses of connected devices |
//! | `pairing_request` | `Option<PairingRequest>` | Pending pairing request |
//!
//! # Control Methods
//!
//! Service-level controls for common operations:
//!
//! - [`enable()`](BluetoothService::enable) / [`disable()`](BluetoothService::disable) -
//!   Power the primary adapter
//! - [`start_discovery()`](BluetoothService::start_discovery) /
//!   [`stop_discovery()`](BluetoothService::stop_discovery) - Scan for devices
//! - [`start_timed_discovery()`](BluetoothService::start_timed_discovery) -
//!   Scan with automatic timeout
//! - Pairing response methods: [`provide_pin()`](BluetoothService::provide_pin),
//!   [`provide_passkey()`](BluetoothService::provide_passkey),
//!   [`provide_confirmation()`](BluetoothService::provide_confirmation),
//!   [`provide_authorization()`](BluetoothService::provide_authorization)
//!
//! Device and adapter instances have their own control methods for direct operations.

mod agent;
/// Bluetooth domain models for adapters and devices.
pub mod core;
mod discovery;
mod error;
mod monitoring;
mod proxy;
mod service;
/// BlueZ type definitions for adapter/device properties.
pub mod types;

pub use error::Error;
pub use service::BluetoothService;
