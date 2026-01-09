use std::path::PathBuf;

use wayle_wallpaper::WallpaperProxy;
use zbus::Connection;

use super::commands::FitModeArg;
use crate::cli::CliAction;

/// Executes the set wallpaper command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails or wallpaper cannot be set.
pub async fn execute(path: PathBuf, fit: Option<FitModeArg>, monitor: Option<String>) -> CliAction {
    let connection = Connection::session()
        .await
        .map_err(|err| format!("Failed to connect to D-Bus: {err}"))?;

    let proxy = WallpaperProxy::new(&connection)
        .await
        .map_err(|err| format!("Failed to connect to wallpaper service: {err}"))?;

    let monitor_arg = monitor.clone().unwrap_or_default();

    if let Some(fit_mode) = fit {
        let mode = match fit_mode {
            FitModeArg::Fill => "fill",
            FitModeArg::Fit => "fit",
            FitModeArg::Center => "center",
            FitModeArg::Tile => "tile",
            FitModeArg::Stretch => "stretch",
        };
        proxy
            .set_fit_mode(mode.to_string())
            .await
            .map_err(|err| format!("Failed to set fit mode: {err}"))?;
    }

    let path_str = path.to_string_lossy().to_string();
    proxy
        .set_wallpaper(path_str, monitor_arg)
        .await
        .map_err(|err| format!("Failed to set wallpaper: {err}"))?;

    match monitor {
        Some(mon) => println!("Wallpaper set to {} on {mon}", path.display()),
        None => println!("Wallpaper set to {}", path.display()),
    }
    Ok(())
}
