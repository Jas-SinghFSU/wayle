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
