//! Bluetooth device management via BlueZ D-Bus.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use wayle_bluetooth::BluetoothService;
//!
//! # async fn example() -> Result<(), wayle_bluetooth::Error> {
//! let bt = BluetoothService::new().await?;
//!
//! // Check adapter state
//! if bt.available.get() {
//!     println!("Bluetooth available, powered: {}", bt.enabled.get());
//! }
//!
//! // List paired devices
//! for device in bt.devices.get().iter() {
//!     let name = device.name.get().unwrap_or_default();
//!     println!("{}: {}", name, device.address.get());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Watching for Changes
//!
//! ```rust,no_run
//! use wayle_bluetooth::BluetoothService;
//! use futures::StreamExt;
//!
//! # async fn example() -> Result<(), wayle_bluetooth::Error> {
//! # let bt = BluetoothService::new().await?;
//! // React to new devices
//! let mut stream = bt.devices.watch();
//! while let Some(devices) = stream.next().await {
//!     println!("Device count: {}", devices.len());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Discovery and Pairing
//!
//! ```rust,no_run
//! # use wayle_bluetooth::BluetoothService;
//! # use std::time::Duration;
//! # async fn example() -> Result<(), wayle_bluetooth::Error> {
//! # let bt = BluetoothService::new().await?;
//! // Scan for 30 seconds
//! bt.start_timed_discovery(Duration::from_secs(30)).await?;
//!
//! // Connect to a device
//! for device in bt.devices.get().iter() {
//!     if device.name.get().as_deref() == Some("My Headphones") {
//!         device.connect().await?;
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Reactive Properties
//!
//! All fields are [`Property<T>`](wayle_common::Property):
//! - `.get()` - Current value snapshot
//! - `.watch()` - Stream yielding on changes
//!
//! # Service Fields
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `adapters` | `Vec<Arc<Adapter>>` | All Bluetooth adapters |
//! | `primary_adapter` | `Option<Arc<Adapter>>` | Active adapter for operations |
//! | `devices` | `Vec<Arc<Device>>` | All discovered devices |
//! | `available` | `bool` | Whether any adapter is present |
//! | `enabled` | `bool` | Whether any adapter is powered |
//! | `connected` | `Vec<String>` | Addresses of connected devices |
//! | `pairing_request` | `Option<PairingRequest>` | Pending pairing request |
//!
//! # Control Methods
//!
//! - [`enable()`](BluetoothService::enable) / [`disable()`](BluetoothService::disable) - Power adapter
//! - [`start_discovery()`](BluetoothService::start_discovery) / [`stop_discovery()`](BluetoothService::stop_discovery) - Scan
//! - [`start_timed_discovery()`](BluetoothService::start_timed_discovery) - Scan with timeout
//!
//! Device-level: `connect()`, `disconnect()`, `pair()`, `forget()`

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
