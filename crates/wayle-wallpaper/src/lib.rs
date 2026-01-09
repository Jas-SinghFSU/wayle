//! Wallpaper service for Wayle desktop shell.
//!
//! Provides wallpaper management with swww backend for efficient rendering,
//! directory cycling, and D-Bus control interface.

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
