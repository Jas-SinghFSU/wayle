//! Reactive media player service for MPRIS D-Bus control and monitoring.
//!
//! # Reactive Property Pattern
//!
//! All state is exposed through [`Property<T>`](wayle_common::Property) fields that support
//! two access patterns:
//!
//! - **Snapshot** (`.get()`): Returns the current value immediately
//! - **Stream** (`.watch()`): Returns a [`Stream`](futures::Stream) that emits on every change
//!
//! ```ignore
//! // Snapshot: get current playback state once
//! let state = player.playback_state.get();
//!
//! // Stream: react to playback state changes
//! player.playback_state.watch().for_each(|state| async move {
//!     println!("State changed: {:?}", state);
//! });
//! ```
//!
//! # Live vs Snapshot Instances
//!
//! Players and metadata can be obtained in two modes:
//!
//! - **Snapshot**: Properties reflect state at creation time and do not update
//! - **Live**: Properties update automatically via D-Bus signal monitoring
//!
//! ```ignore
//! // Snapshot - frozen state
//! let player = service.player(&player_id).await?;
//!
//! // Live - auto-updating properties
//! let player = service.player_monitored(&player_id).await?;
//! ```
//!
//! # Playback Control
//!
//! [`Player`](core::player::Player) exposes async methods for controlling playback:
//!
//! - `play_pause()`, `next()`, `previous()`
//! - `seek()`, `set_position()`
//! - `set_volume()`, `set_loop_mode()`, `set_shuffle_mode()`
//! - `toggle_loop()`, `toggle_shuffle()`
//!
//! # Service Fields
//!
//! [`MediaService`] tracks all MPRIS players on the session bus:
//!
//! - `player_list`: All discovered players (filtered by `ignored_patterns`)
//! - `active_player`: Currently selected player for focused control

mod builder;
/// Core media domain models
pub mod core;
mod dbus;
mod error;
mod monitoring;
mod proxy;
mod service;
/// Type definitions for media service configuration, states, and identifiers
pub mod types;

pub use builder::MediaServiceBuilder;
pub use dbus::{MediaProxy, SERVICE_NAME, SERVICE_PATH};
pub use error::Error;
pub use service::MediaService;
