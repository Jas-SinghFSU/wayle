//! System tray management service for StatusNotifier items.
//!
//! This crate provides a service for managing system tray items through the
//! StatusNotifier/SNI protocol and DBusMenu specification. It monitors tray
//! items, handles menu interactions, and provides reactive streams for tray
//! item events and state changes.

/// UI framework adapters for the systray service.
///
/// These adapters enable integration with various GUI toolkits,
/// allowing systray menus to be displayed natively in different frameworks.
pub mod adapters;
mod builder;
/// Core types and functionality for system tray items
pub mod core;
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
pub use service::SystemTrayService;
