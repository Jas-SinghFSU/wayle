//! Power profiles service for managing system power modes.
//!
//! Provides integration with the power-profiles-daemon D-Bus service
//! to monitor and control system power profiles (performance, balanced, power-saver).

/// Core power profiles domain models
pub mod core;
mod error;
mod proxy;
mod service;
/// Power profiles type definitions  
pub mod types;

pub use core::PowerProfiles;

pub use error::Error;
pub use service::PowerProfilesService;
