mod agent;
/// Core domain models for Bluetooth service components.
mod core;
/// Bluetooth device discovery functionality.
pub mod discovery;
/// Bluetooth service error types.
pub mod error;
/// Background monitoring for Bluetooth adapters and devices.
pub mod monitoring;
/// D-Bus proxy implementations for BlueZ interfaces.
mod proxy;
/// Bluetooth service implementation.
pub mod service;
/// Type definitions for Bluetooth enums, flags, and states.
pub mod types;

pub use error::BluetoothError;
