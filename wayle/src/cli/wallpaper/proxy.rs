//! D-Bus proxy utilities for wallpaper commands.

use wayle_wallpaper::WallpaperProxy;
use zbus::{Connection, Error as ZbusError};

use crate::cli::dbus;

const SERVICE_NAME: &str = "Wallpaper";

/// Creates a WallpaperProxy connection.
///
/// # Errors
/// Returns error if D-Bus connection or proxy creation fails.
pub async fn connect() -> Result<(Connection, WallpaperProxy<'static>), String> {
    let connection = dbus::session().await?;

    let proxy = WallpaperProxy::new(&connection)
        .await
        .map_err(|e| format!("Failed to create wallpaper proxy: {e}"))?;

    Ok((connection, proxy))
}

/// Transforms zbus errors into user-friendly messages.
pub fn format_error(operation: &str, error: ZbusError) -> String {
    dbus::format_error(SERVICE_NAME, operation, error)
}
