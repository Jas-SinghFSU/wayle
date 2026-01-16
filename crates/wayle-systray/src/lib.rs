//! System tray management via StatusNotifier/SNI protocol and DBusMenu.
//!
//! Monitors tray items, handles menu interactions, and provides reactive streams
//! for item events and state changes.

/// UI framework adapters (GTK4, etc.) for native systray menu rendering.
pub mod adapters;
mod builder;
/// Core types and functionality for system tray items
pub mod core;
/// D-Bus interface for external control.
pub mod dbus;
mod discovery;
/// Error types for the system tray service
pub mod error;
mod events;
mod monitoring;
mod proxy;
/// Main system tray service implementation
pub mod service;
/// Type definitions for StatusNotifier and DBusMenu protocols
pub mod types;
mod watcher;

pub use builder::SystemTrayServiceBuilder;
pub use dbus::SystemTrayWayleProxy;
pub use service::SystemTrayService;
