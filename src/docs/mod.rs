//! Documentation generation for configuration schemas.
//!
//! Automatically generates markdown documentation from Rust configuration
//! structures using reflection and type information.

/// Documentation generator
pub mod generator;
/// Markdown generation
pub mod markdown;
/// Module documentation
pub mod module;
/// Module registry
pub mod registry;
/// Schema documentation
pub mod schema;
