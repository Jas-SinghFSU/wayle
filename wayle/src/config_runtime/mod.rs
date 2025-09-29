//! Reactive configuration store with change tracking.
//!
//! Provides a configuration store that can load TOML files,
//! track changes, and notify subscribers of configuration updates.

/// Configuration broadcast system
pub mod broadcast;
/// Configuration change tracking
pub mod changes;
/// Configuration diffing
pub mod diff;
/// File system watching
pub mod file_watching;
/// Path operations
pub mod path_ops;
/// Configuration runtime
pub mod runtime;

#[cfg(test)]
/// Configuration runtime tests
pub mod tests;

pub use changes::Error;
