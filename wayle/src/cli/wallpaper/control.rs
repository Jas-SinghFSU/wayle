use wayle_wallpaper::WallpaperProxy;
use zbus::Connection;

use crate::cli::CliAction;

/// Executes the stop command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails.
pub async fn stop() -> CliAction {
    let proxy = connect_proxy().await?;
    proxy
        .stop_cycling()
        .await
        .map_err(|err| format!("Failed to stop cycling: {err}"))?;

    println!("Wallpaper cycling stopped");
    Ok(())
}

/// Executes the next command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails.
pub async fn next() -> CliAction {
    let proxy = connect_proxy().await?;
    proxy
        .next()
        .await
        .map_err(|err| format!("Failed to advance: {err}"))?;

    println!("Advanced to next wallpaper");
    Ok(())
}

/// Executes the previous command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails.
pub async fn previous() -> CliAction {
    let proxy = connect_proxy().await?;
    proxy
        .previous()
        .await
        .map_err(|err| format!("Failed to go back: {err}"))?;

    println!("Went back to previous wallpaper");
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
