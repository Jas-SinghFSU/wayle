//! System tray management via the StatusNotifier (SNI) and DBusMenu protocols.
//!
//! # Overview
//!
//! The service discovers and monitors system tray items registered on D-Bus,
//! providing reactive access to item properties, icons, and menus. It can operate
//! as either a StatusNotifierWatcher (central registry) or a StatusNotifierHost
//! (consumer of items from an existing watcher).
//!
//! # Reactive Pattern
//!
//! All tray item properties use [`Property<T>`](wayle_common::Property):
//! - `.get()` returns the current value snapshot
//! - `.watch()` returns a stream of value changes
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use wayle_systray::SystemTrayService;
//!
//! # async fn example() -> Result<(), wayle_systray::error::Error> {
//! let service = SystemTrayService::new().await?;
//!
//! // Get current items
//! for item in service.items.get().iter() {
//!     println!("{}: {}", item.id.get(), item.title.get());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Watching for Changes
//!
//! ```rust,no_run
//! use wayle_systray::SystemTrayService;
//! use futures::StreamExt;
//!
//! # async fn example() -> Result<(), wayle_systray::error::Error> {
//! # let service = SystemTrayService::new().await?;
//! // React to tray item changes
//! let mut stream = service.items.watch();
//! while let Some(items) = stream.next().await {
//!     println!("Tray items changed: {} items", items.len());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Configuration
//!
//! | Method | Effect |
//! |--------|--------|
//! | `with_daemon()` | Interact with tray items from scripts or other processes |
//! | `mode(TrayMode)` | Set operating mode: `Watcher`, `Host`, or `Auto` (default) |
//!
//! ```rust,no_run
//! use wayle_systray::{SystemTrayService, types::TrayMode};
//!
//! # async fn example() -> Result<(), wayle_systray::error::Error> {
//! let tray = SystemTrayService::builder()
//!     .with_daemon()
//!     .mode(TrayMode::Auto)
//!     .build()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Service Fields
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `is_watcher` | `bool` | Whether operating as the watcher registry |
//! | `items` | `Property<Vec<Arc<TrayItem>>>` | Currently registered tray items |

/// UI framework adapters (GTK4) for native systray menu rendering.
pub mod adapters;
mod builder;
/// System tray item model.
pub mod core;
/// D-Bus interface for CLI control.
pub mod dbus;
mod discovery;
/// Error types.
pub mod error;
mod events;
mod monitoring;
mod proxy;
/// Main service implementation.
pub mod service;
/// SNI and DBusMenu protocol types.
pub mod types;
mod watcher;

pub use builder::SystemTrayServiceBuilder;
pub use dbus::SystemTrayWayleProxy;
pub use service::SystemTrayService;
