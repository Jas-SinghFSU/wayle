//! Common utilities and types shared across Wayle services.
//!
//! Provides reusable components including property watchers, macros,
//! runtime state management, and the global service registry.

mod diagnostic;
#[macro_use]
/// Common macros for services
pub mod macros;
/// Shell command execution utilities.
pub mod process;
mod property;
/// Global service registry for dependency injection.
pub mod services;
/// Shared constants for wayle-shell IPC.
pub mod shell;
mod state;
/// Ergonomic watcher utilities for Relm4 components.
pub mod watchers;

pub use diagnostic::Diagnostic;
pub use property::{
    ApplyConfigLayer, ApplyRuntimeLayer, ClearRuntimeByPath, CommitConfigReload, ComputedProperty,
    ConfigProperty, ExtractRuntimeValues, Property, PropertyStream, ResetConfigLayer,
    ResetRuntimeLayer, SubscribeChanges, ValueSource,
};
pub use state::RuntimeState;

/// Root path for service object paths in D-Bus hierarchy.
pub const ROOT_PATH: &str = "/";
/// Null path used when no specific object path is required.
pub const NULL_PATH: &str = "/";
