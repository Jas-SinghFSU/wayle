//! Wallpaper management via awww with cycling and theming support.
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
//! for (monitor, state) in wp.monitors.get().iter() {
//!     println!("{}: {:?} (fit: {})", monitor, state.wallpaper, state.fit_mode);
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
//! )?;
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
//! | `transition(TransitionConfig)` | Animation when changing wallpapers |
//! | `color_extractor(ColorExtractorConfig)` | Tool and parameters for extracting dominant colors |
//! | `theming_monitor(Option<String>)` | Which monitor drives color extraction |
//! | `shared_cycle(bool)` | Sync cycling across monitors in shuffle mode |
//! | `engine_active(bool)` | Toggle awww rendering (state tracking continues) |
//!
//! ```rust,no_run
//! use wayle_wallpaper::{WallpaperService, TransitionConfig, TransitionType};
//!
//! # async fn example() -> Result<(), wayle_wallpaper::Error> {
//! let wp = WallpaperService::builder()
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
//! | `cycling` | `Option<CyclingConfig>` | Active cycling state |
//! | `monitors` | `HashMap<String, MonitorState>` | Per-monitor wallpaper and fit mode |
//! | `transition` | [`TransitionConfig`] | Animation settings |
//!
//! # Control Methods
//!
//! - `set_wallpaper()` - Apply wallpaper to one or all monitors
//! - `start_cycling()` / `stop_cycling()` - Directory cycling
//! - `advance_cycle()` / `rewind_cycle()` - Manual navigation
//! - `set_fit_mode()` - Change scaling mode per monitor or globally
//! - `set_transition()` - Configure animations
//!
//! # D-Bus Interface
//!
//! When `with_daemon()` is enabled, the service registers on the session bus.
//!
//! - **Service:** `com.wayle.Wallpaper1`
//! - **Path:** `/com/wayle/Wallpaper`
//! - **Interface:** `com.wayle.Wallpaper1`
//!
//! See [`dbus.md`](https://github.com/Jas-SinghFSU/wayle/blob/master/crates/wayle-wallpaper/dbus.md) for the full interface specification.

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
pub use types::{
    ColorExtractor, ColorExtractorConfig, CyclingConfig, CyclingMode, FitMode, MonitorState,
};
