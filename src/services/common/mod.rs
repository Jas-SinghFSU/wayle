//! Common utilities and abstractions for services

/// Generic D-Bus ObjectManager interface for service discovery
pub mod object_manager;
/// Reactive property system for fine-grained state updates
pub mod property;
// Service macros
#[macro_use]
mod macros;

pub use object_manager::*;
pub use property::{ComputedProperty, Property};
