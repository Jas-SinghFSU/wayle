//! Reactive bindings to Hyprland compositor state via IPC.
//!
//! # Overview
//!
//! Connects to Hyprland's Unix domain sockets to query state and receive events.
//! State is exposed through reactive [`Property`] fields that update automatically
//! when Hyprland emits relevant events.
//!
//! # Reactive Properties
//!
//! All state fields use [`Property<T>`] which supports two access patterns:
//!
//! - **Snapshot**: `property.get()` returns the current value
//! - **Stream**: `property.watch()` returns a stream that yields on changes
//!
//! ```no_run
//! # use wayle_hyprland::HyprlandService;
//! # use futures::StreamExt;
//! # async fn example() -> wayle_hyprland::Result<()> {
//! let service = HyprlandService::new().await?;
//!
//! // Get current workspaces
//! let workspaces = service.workspaces.get();
//!
//! // Watch for workspace changes
//! let mut stream = service.workspaces.watch();
//! while let Some(workspaces) = stream.next().await {
//!     println!("Workspaces changed: {:?}", workspaces.len());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Service Fields
//!
//! The [`HyprlandService`] exposes:
//!
//! - `workspaces` - All workspaces (normal and special)
//! - `clients` - All open windows
//! - `monitors` - Connected displays
//! - `layers` - Layer shell surfaces (panels, overlays, etc.)
//!
//! # Event Streaming
//!
//! Raw Hyprland events can be streamed via [`HyprlandService::events()`]:
//!
//! ```no_run
//! # use wayle_hyprland::HyprlandService;
//! # use futures::StreamExt;
//! # async fn example() -> wayle_hyprland::Result<()> {
//! let service = HyprlandService::new().await?;
//! let mut events = service.events();
//!
//! while let Some(event) = events.next().await {
//!     println!("Event: {:?}", event);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # IPC Commands
//!
//! Execute Hyprland commands via [`HyprlandService::dispatch()`]:
//!
//! ```no_run
//! # use wayle_hyprland::HyprlandService;
//! # async fn example() -> wayle_hyprland::Result<()> {
//! let service = HyprlandService::new().await?;
//! service.dispatch("workspace 1").await?;
//! # Ok(())
//! # }
//! ```
//!
//! [`Property`]: wayle_common::Property
//! [`Property<T>`]: wayle_common::Property

mod core;
mod discovery;
mod error;
mod ipc;
mod monitoring;
mod service;
mod types;

pub use core::{client::Client, monitor::Monitor, workspace::Workspace};

pub use error::{Error, Result};
pub use ipc::events::types::HyprlandEvent;
pub use service::HyprlandService;
pub(crate) use types::*;
pub use types::{
    Address, BindData, CursorPosition, DeviceInfo, FocusHistoryId, MonitorId, ProcessId,
    ScreencastOwner, WorkspaceId, WorkspaceInfo, WorkspaceRule,
};
