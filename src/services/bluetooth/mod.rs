mod agent;
/// Core domain models for Bluetooth service components.
mod core;
pub mod discovery;
/// Bluetooth service error types.
pub mod error;
pub mod monitoring;
/// D-Bus proxy implementations for BlueZ interfaces.
mod proxy;
pub mod service;
/// Type definitions for Bluetooth enums, flags, and states.
pub mod types;

pub use error::BluetoothError;
