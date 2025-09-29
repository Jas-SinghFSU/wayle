//! Common utilities and types shared across Wayle services.
//!
//! Provides reusable components including property watchers, macros,
//! and runtime state management for service persistence.

#[macro_use]
/// Common macros for services
pub mod macros;
mod property;
mod state;

pub use property::{ComputedProperty, Property, PropertyStream};
pub use state::RuntimeState;

/// Root path for service object paths in D-Bus hierarchy.
pub const ROOT_PATH: &str = "/";
/// Null path used when no specific object path is required.
pub const NULL_PATH: &str = "/";
