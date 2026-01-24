//! MPRIS media player control via D-Bus.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use wayle_media::MediaService;
//!
//! # async fn example() -> Result<(), wayle_media::Error> {
//! let media = MediaService::new().await?;
//!
//! // Get active player
//! if let Some(player) = media.active_player.get() {
//!     println!("{}: {:?}", player.identity.get(), player.playback_state.get());
//!     println!("Track: {}", player.metadata.title.get());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Watching for Changes
//!
//! ```rust,no_run
//! use wayle_media::MediaService;
//! use futures::StreamExt;
//!
//! # async fn example() -> Result<(), wayle_media::Error> {
//! # let media = MediaService::new().await?;
//! // React to player list changes
//! let mut stream = media.player_list.watch();
//! while let Some(players) = stream.next().await {
//!     println!("{} players available", players.len());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Playback Control
//!
//! ```rust,no_run
//! # use wayle_media::MediaService;
//! # async fn example() -> Result<(), wayle_media::Error> {
//! # let media = MediaService::new().await?;
//! if let Some(player) = media.active_player.get() {
//!     player.play_pause().await?;
//!     player.next().await?;
//!     player.set_volume(0.5.into()).await?;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Configuration
//!
//! | Method | Effect |
//! |--------|--------|
//! | `with_daemon()` | Control playback from scripts or other processes |
//! | `ignore_player(pattern)` | Skip players matching the pattern |
//!
//! ```rust,no_run
//! use wayle_media::MediaService;
//!
//! # async fn example() -> Result<(), wayle_media::Error> {
//! let media = MediaService::builder()
//!     .with_daemon()
//!     .ignore_player("chromium".to_string())
//!     .ignore_player("firefox".to_string())
//!     .build()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # D-Bus Interface
//!
//! When `with_daemon()` is enabled, the service registers on the session bus.
//!
//! - **Service:** `com.wayle.Media1`
//! - **Path:** `/com/wayle/Media`
//! - **Interface:** `com.wayle.Media1`
//!
//! See [`dbus.md`](https://github.com/Jas-SinghFSU/wayle/blob/master/crates/wayle-media/dbus.md) for the full interface specification.
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
//! | `player_list` | `Vec<Arc<Player>>` | All MPRIS players |
//! | `active_player` | `Option<Arc<Player>>` | Selected player for control |
//!
//! # Control Methods
//!
//! On [`Player`](core::player::Player):
//! - `play_pause()`, `next()`, `previous()` - Playback
//! - `seek()`, `set_position()` - Position
//! - `set_volume()`, `set_loop_mode()`, `set_shuffle_mode()` - Settings

mod builder;
/// Core media domain models
pub mod core;
mod dbus;
mod error;
mod monitoring;
mod proxy;
mod selection;
mod service;
/// Type definitions for media service configuration, states, and identifiers
pub mod types;

pub use builder::MediaServiceBuilder;
pub use dbus::{MediaProxy, SERVICE_NAME, SERVICE_PATH};
pub use error::Error;
pub use service::MediaService;
