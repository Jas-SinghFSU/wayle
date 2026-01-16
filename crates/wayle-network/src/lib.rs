//! Network device management and monitoring via NetworkManager D-Bus API.
//!
//! Monitors WiFi and wired connections, device availability, and network events.
//! Exposes device information and state changes through a reactive stream-based API.

/// Core network domain models
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
