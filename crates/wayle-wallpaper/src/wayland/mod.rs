//! Wayland output discovery and monitoring.
//!
//! Provides compositor-agnostic monitor detection using the standard
//! Wayland `wl_output` protocol. Works on any Wayland compositor.

mod listener;

pub use listener::{OutputEvent, OutputWatcher};
