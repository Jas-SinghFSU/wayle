//! Wallpaper management via swww with cycling and theming support.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use wayle_wallpaper::WallpaperService;
//! use std::path::PathBuf;
//!
//! # async fn example() -> Result<(), wayle_wallpaper::Error> {
//! let wp = WallpaperService::new().await?;
//!
//! // Set wallpaper on all monitors
//! wp.set_wallpaper(PathBuf::from("/path/to/image.jpg"), None).await?;
//!
//! // Check current state
//! println!("Fit mode: {:?}", wp.fit_mode.get());
//! for (monitor, state) in wp.monitors.get().iter() {
//!     println!("{}: {:?}", monitor, state.wallpaper);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Directory Cycling
//!
//! ```rust,no_run
//! # use wayle_wallpaper::{WallpaperService, CyclingMode};
//! # use std::path::PathBuf;
//! # use std::time::Duration;
//! # async fn example() -> Result<(), wayle_wallpaper::Error> {
//! # let wp = WallpaperService::new().await?;
//! // Cycle through a directory
//! wp.start_cycling(
//!     PathBuf::from("/path/to/wallpapers"),
//!     Duration::from_secs(300),
//!     CyclingMode::Sequential,
//! ).await?;
//!
//! // Manual navigation
//! wp.advance_cycle().await?;
//! wp.rewind_cycle().await?;
//!
//! wp.stop_cycling();
//! # Ok(())
//! # }
//! ```
//!
//! # Configuration
//!
//! | Method | Effect |
//! |--------|--------|
//! | `fit_mode(FitMode)` | How images scale to fit the screen |
//! | `transition(TransitionConfig)` | Animation when changing wallpapers |
//! | `color_extractor(ColorExtractor)` | Tool for extracting dominant colors |
//!
//! ```rust,no_run
//! use wayle_wallpaper::{WallpaperService, FitMode, TransitionConfig, TransitionType};
//!
//! # async fn example() -> Result<(), wayle_wallpaper::Error> {
//! let wp = WallpaperService::builder()
//!     .fit_mode(FitMode::Fill)
//!     .transition(TransitionConfig {
//!         transition_type: TransitionType::Left,
//!         ..Default::default()
//!     })
//!     .build()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Reactive Properties
//!
//! All fields are [`Property<T>`](wayle_common::Property):
//! - `.get()` - Current value snapshot
//! - `.watch()` - Stream yielding on changes
//!
//! # Service Fields
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `fit_mode` | [`FitMode`] | Image scaling mode |
//! | `cycling` | `Option<CyclingConfig>` | Active cycling state |
//! | `monitors` | `HashMap<String, MonitorState>` | Per-monitor wallpaper state |
//! | `transition` | [`TransitionConfig`] | Animation settings |
//!
//! # Control Methods
//!
//! - `set_wallpaper()` - Apply wallpaper to one or all monitors
//! - `start_cycling()` / `stop_cycling()` - Directory cycling
//! - `advance_cycle()` / `rewind_cycle()` - Manual navigation
//! - `set_fit_mode()` - Change scaling mode
//! - `set_transition()` - Configure animations

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
