#[macro_use]
/// Common macros for services
pub mod macros;
mod property;

pub use property::{ComputedProperty, Property, PropertyStream};

pub(crate) const ROOT_PATH: &str = "/";
pub(crate) const NULL_PATH: &str = "/";
