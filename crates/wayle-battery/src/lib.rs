//! Battery monitoring via UPower D-Bus interface.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use wayle_battery::BatteryService;
//!
//! # async fn example() -> Result<(), wayle_battery::Error> {
//! let service = BatteryService::new().await?;
//!
//! // Access battery state
//! let percentage = service.device.percentage.get();
//! let state = service.device.state.get();
//!
//! println!("Battery: {percentage}% ({state})");
//! # Ok(())
//! # }
//! ```
//!
//! # Reactive Properties
//!
//! All device properties are wrapped in [`Property<T>`](wayle_common::Property):
//! - **Snapshot**: `.get()` returns the current value
//! - **Stream**: `.watch()` yields on every change
//!
//! ```rust,no_run
//! # use wayle_battery::BatteryService;
//! # use futures::StreamExt;
//! # async fn example() -> Result<(), wayle_battery::Error> {
//! # let service = BatteryService::new().await?;
//! // React to state changes
//! let mut state_stream = service.device.state.watch();
//! while let Some(new_state) = state_stream.next().await {
//!     println!("State changed: {new_state}");
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # DisplayDevice vs Specific Devices
//!
//! By default, [`BatteryService::new`] monitors UPower's DisplayDevice - a composite
//! that aggregates all batteries. For specific device monitoring:
//!
//! ```rust,no_run
//! # use wayle_battery::BatteryService;
//! use zbus::zvariant::OwnedObjectPath;
//!
//! # async fn example() -> Result<(), wayle_battery::Error> {
//! let path = OwnedObjectPath::try_from("/org/freedesktop/UPower/devices/battery_BAT0")?;
//! let service = BatteryService::builder()
//!     .device_path(path)
//!     .build()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Control Methods
//!
//! The [`Device`](core::device::Device) type exposes UPower operations:
//! - [`refresh`](core::device::Device::refresh) - Force data refresh from hardware
//! - [`get_history`](core::device::Device::get_history) - Historical charge/rate data
//! - [`get_statistics`](core::device::Device::get_statistics) - Charge/discharge statistics
//! - [`enable_charge_threshold`](core::device::Device::enable_charge_threshold) - Battery charge limiting

mod builder;
/// Core battery device functionality.
pub mod core;
mod error;
mod proxy;
mod service;
/// Type definitions for battery service domain models and enums.
pub mod types;

pub use builder::BatteryServiceBuilder;
pub use error::Error;
pub use service::BatteryService;
