//! Wallpaper management with swww backend, directory cycling, and D-Bus control.
//!
//! # Service Access Pattern
//!
//! All state is exposed through [`Property`] fields on [`WallpaperService`].
//! Use `.get()` for point-in-time snapshots, `.watch()` for reactive streams:
//!
//! ```ignore
//! let service = WallpaperService::new().await?;
//!
//! // Snapshot: current fit mode
//! let mode = service.fit_mode.get();
//!
//! // Stream: react to cycling config changes
//! service.cycling.watch().for_each(|config| async {
//!     // handle change
//! });
//! ```
//!
//! # Control Methods
//!
//! - [`WallpaperService::set_wallpaper`] - Apply wallpaper to one or all monitors
//! - [`WallpaperService::start_cycling`] - Begin cycling through a directory
//! - [`WallpaperService::stop_cycling`] - Stop cycling
//! - [`WallpaperService::advance_cycle`] / [`WallpaperService::rewind_cycle`] - Manual navigation
//! - [`WallpaperService::set_fit_mode`] - Change scaling mode (re-renders all monitors)
//! - [`WallpaperService::set_transition`] - Configure transition animations
//!
//! # Service Fields
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `fit_mode` | [`FitMode`] | Image scaling mode |
//! | `cycling` | `Option<CyclingConfig>` | Active cycling state |
//! | `monitors` | `HashMap<String, MonitorState>` | Per-monitor wallpaper state |
//! | `theming_monitor` | `Option<String>` | Monitor used for color extraction |
//! | `color_extractor` | [`ColorExtractor`] | Extraction tool (wallust, matugen, pywal) |
//! | `transition` | [`TransitionConfig`] | Animation settings |
//!
//! # D-Bus Interface
//!
//! Exposes controls at `org.wayle.Wallpaper` on the session bus.
//! Use [`WallpaperProxy`] for client access.
//!
//! [`Property`]: wayle_common::Property

mod backend;
mod builder;
mod dbus;
pub mod error;
mod service;
mod tasks;
pub mod types;
mod wayland;

pub use backend::{
    BezierCurve, Position, TransitionAngle, TransitionConfig, TransitionDuration, TransitionFps,
    TransitionStep, TransitionType, WaveDimensions,
};
pub use builder::WallpaperServiceBuilder;
pub use dbus::{SERVICE_NAME, SERVICE_PATH, WallpaperProxy};
pub use error::Error;
pub use service::WallpaperService;
pub use types::{ColorExtractor, CyclingConfig, CyclingMode, FitMode, MonitorState};
