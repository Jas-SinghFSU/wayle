use std::path::PathBuf;

use super::{
    commands::FitModeArg,
    proxy::{connect, format_error},
};
use crate::cli::CliAction;

/// Executes the set wallpaper command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails or wallpaper cannot be set.
pub async fn execute(path: PathBuf, fit: Option<FitModeArg>, monitor: Option<String>) -> CliAction {
    let (_connection, proxy) = connect().await?;

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
            .set_fit_mode(mode.to_string(), monitor_arg.clone())
            .await
            .map_err(|e| format_error("set fit mode", e))?;
    }

    let path_str = path.to_string_lossy().to_string();
    proxy
        .set_wallpaper(path_str, monitor_arg)
        .await
        .map_err(|e| format_error("set wallpaper", e))?;

    match monitor {
        Some(mon) => println!("Wallpaper set to {} on {mon}", path.display()),
        None => println!("Wallpaper set to {}", path.display()),
    }
    Ok(())
}
