//! Hyprland window manager integration for Wayle.
//!
//! Provides reactive bindings to Hyprland's state through its IPC protocol,
//! including monitors, workspaces, windows, and layer surfaces. Events are
//! streamed from Hyprland's event socket and exposed as typed notifications.
//!
//! # Example
//!
//! ```no_run
//! use wayle_hyprland::HyprlandService;
//!
//! # async fn example() {
//! let service = HyprlandService::new();
//! # }
//! ```

mod core;
mod error;
mod events;
mod service;
mod types;

pub use error::{Error, Result};
pub use events::types::{HyprlandEvent, ServiceNotification};
pub use service::HyprlandService;
pub use types::*;
