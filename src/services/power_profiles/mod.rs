/// Core power profiles domain models
pub mod core;
/// Power profiles error types
pub mod error;
/// D-Bus proxy interfaces for power-profiles-daemon
pub mod proxy;
/// High-level power profiles service interface
pub mod service;
/// Power profiles type definitions
pub mod types;

pub use error::Error;
