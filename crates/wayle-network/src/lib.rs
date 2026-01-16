//! Network management via NetworkManager D-Bus API.
//!
//! # Overview
//!
//! Provides WiFi and wired network monitoring, connection management, and device
//! tracking through NetworkManager. All network state is exposed as reactive
//! [`Property`] types that can be polled or streamed.
//!
//! # Reactive Pattern
//!
//! Network state uses [`Property<T>`] for reactive access:
//!
//! - **Snapshot**: Call `.get()` for the current value
//! - **Stream**: Call `.watch()` for a stream of changes
//!
//! ```ignore
//! // Get current WiFi enabled state
//! let enabled = service.wifi.as_ref().map(|w| w.enabled.get());
//!
//! // Watch for connectivity changes
//! let stream = service.wifi.as_ref().map(|w| w.connectivity.watch());
//! ```
//!
//! # Live vs Snapshot Instances
//!
//! Most types implement the `Reactive` trait with two constructors:
//!
//! - **`get()`**: Returns an owned snapshot. Properties are populated but not monitored.
//! - **`get_live()`**: Returns `Arc<T>` with active monitoring. Properties update automatically
//!   as NetworkManager state changes.
//!
//! The service methods mirror this pattern:
//!
//! ```ignore
//! // Snapshot - read once, no updates
//! let device = service.device(path).await?;
//!
//! // Live - properties update automatically
//! let device = service.device_monitored(path).await?;
//! ```
//!
//! # Service Fields
//!
//! [`NetworkService`] exposes:
//!
//! - `settings` - Connection profile management (add, remove, list saved networks)
//! - `wifi` - WiFi device state, access points, and connection control (if available)
//! - `wired` - Ethernet device state and connectivity (if available)
//! - `primary` - Which interface provides the primary connection
//!
//! # Control Methods
//!
//! WiFi operations are exposed on the [`Wifi`] struct:
//!
//! - `set_enabled()` - Enable/disable WiFi system-wide
//! - `connect()` - Connect to an access point
//! - `disconnect()` - Disconnect from current network
//!
//! Connection profile management is on [`Settings`](core::settings::Settings):
//!
//! - `add_connection()` - Create new connection profile
//! - `list_connections()` - List saved profiles
//! - `connections_for_ssid()` - Find profiles matching an SSID
//!
//! [`Property`]: wayle_common::Property
//! [`Property<T>`]: wayle_common::Property
//! [`Wifi`]: wifi::Wifi

/// Core network domain models.
pub mod core;
mod discovery;
mod error;
mod monitoring;
mod proxy;
mod service;
/// Network type definitions
pub mod types;
/// WiFi device functionality
pub mod wifi;
/// Wired device functionality
pub mod wired;

pub use error::Error;
pub use service::NetworkService;
