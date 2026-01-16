mod client;
mod server;

pub use client::WallpaperProxy;
pub(crate) use server::WallpaperDaemon;

/// D-Bus service name.
pub const SERVICE_NAME: &str = "com.wayle.Wallpaper1";

/// D-Bus object path.
pub const SERVICE_PATH: &str = "/com/wayle/Wallpaper";
