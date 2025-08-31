//! Common utilities and abstractions for services

/// Reactive property system for fine-grained state updates
pub mod property;
// Service macros
#[macro_use]
mod macros;

pub use property::{ComputedProperty, Property};

/// D-Bus root object path for object hierarchy traversal.
///
/// This represents the root of the D-Bus object tree and is used when you need to
/// discover or access all objects under a service. Commonly used with `ObjectManagerProxy`
/// to enumerate all available objects in a service's hierarchy.
pub(crate) const ROOT_PATH: &str = "/";

/// D-Bus null object path sentinel indicating "no object" or empty reference.
/// Used for semantic clarity
pub const NULL_PATH: &str = "/";
