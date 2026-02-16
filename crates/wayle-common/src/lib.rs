//! Common utilities and types shared across Wayle services.
//!
//! Provides reusable components including property watchers, macros,
//! and runtime state management.

mod diagnostic;
/// Glob pattern matching utilities.
pub mod glob;
/// D-Bus client proxy for idle inhibit service.
pub mod idle_inhibit;
#[macro_use]
/// Common macros for services
pub mod macros;
/// Shell command execution utilities.
pub mod process;
mod property;
/// Shared constants for wayle-shell IPC.
pub mod shell;
mod state;
/// Format string rendering with Jinja2 syntax and JSONPath support.
pub mod template;
/// Ergonomic watcher utilities for Relm4 components.
pub mod watchers;

pub use diagnostic::Diagnostic;
pub use property::{
    ApplyConfigLayer, ApplyRuntimeLayer, ClearRuntimeByPath, CommitConfigReload, ComputedProperty,
    ConfigProperty, ExtractRuntimeValues, Property, PropertyStream, ResetConfigLayer,
    ResetRuntimeLayer, SubscribeChanges, ValueSource,
};
pub use state::RuntimeState;
pub use watchers::WatcherToken;

/// Root path for service object paths in D-Bus hierarchy.
pub const ROOT_PATH: &str = "/";
/// Null path used when no specific object path is required.
pub const NULL_PATH: &str = "/";
