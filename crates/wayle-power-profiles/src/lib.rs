//! Power profiles service for managing system power modes.
//!
//! Provides integration with the power-profiles-daemon D-Bus service
//! to monitor and control system power profiles (performance, balanced, power-saver).

mod builder;
/// Core power profiles domain models
pub mod core;
/// D-Bus interface for external control.
pub mod dbus;
mod error;
mod proxy;
mod service;
/// Power profiles type definitions
pub mod types;

pub use core::PowerProfiles;

pub use builder::PowerProfilesServiceBuilder;
pub use error::Error;
pub use service::PowerProfilesService;
