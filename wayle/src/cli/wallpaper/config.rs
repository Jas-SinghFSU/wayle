use wayle_wallpaper::WallpaperProxy;
use zbus::Connection;

use crate::cli::CliAction;

/// Executes the theming-monitor command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails.
pub async fn set_theming_monitor(monitor: String) -> CliAction {
    let proxy = connect_proxy().await?;
    proxy
        .set_theming_monitor(monitor.clone())
        .await
        .map_err(|err| format!("Failed to set theming monitor: {err}"))?;

    if monitor.is_empty() {
        println!("Theming monitor: default");
    } else {
        println!("Theming monitor: {monitor}");
    }
    Ok(())
}

async fn connect_proxy() -> Result<WallpaperProxy<'static>, String> {
    let connection = Connection::session()
        .await
        .map_err(|err| format!("Failed to connect to D-Bus: {err}"))?;

    WallpaperProxy::new(&connection)
        .await
        .map_err(|err| format!("Failed to connect to wallpaper service: {err}"))
}
