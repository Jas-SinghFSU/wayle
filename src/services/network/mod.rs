/// Core network domain models
pub mod core;
/// Network device discovery functionality
pub mod discovery;
/// Network error types
pub mod error;
pub(crate) mod monitoring;
/// D-Bus proxy interfaces for NetworkManager
pub mod proxy;
/// Network service implementation
pub mod service;
/// Network type definitions
pub mod types;
/// WiFi device functionality
pub mod wifi;
/// Wired device functionality
pub mod wired;

pub use error::Error;
