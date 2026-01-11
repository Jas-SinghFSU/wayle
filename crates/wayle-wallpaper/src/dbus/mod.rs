//! D-Bus interface for the wallpaper service.
//!
//! Contains the server-side daemon interface and client-side proxy.

mod client;
mod server;

pub use client::WallpaperProxy;
pub(crate) use server::WallpaperDaemon;

/// D-Bus service name.
pub const SERVICE_NAME: &str = "com.wayle.Wallpaper1";

/// D-Bus object path.
pub const SERVICE_PATH: &str = "/com/wayle/Wallpaper";
