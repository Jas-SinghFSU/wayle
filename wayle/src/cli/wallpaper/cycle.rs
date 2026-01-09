use std::path::PathBuf;

use wayle_wallpaper::WallpaperProxy;
use zbus::Connection;

use super::commands::CyclingModeArg;
use crate::cli::CliAction;

/// Executes the cycle command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails or directory is invalid.
pub async fn execute(directory: PathBuf, interval: u32, mode: CyclingModeArg) -> CliAction {
    let connection = Connection::session()
        .await
        .map_err(|err| format!("Failed to connect to D-Bus: {err}"))?;

    let proxy = WallpaperProxy::new(&connection)
        .await
        .map_err(|err| format!("Failed to connect to wallpaper service: {err}"))?;

    let mode_str = match mode {
        CyclingModeArg::Sequential => "sequential",
        CyclingModeArg::Shuffle => "shuffle",
    };

    let dir_str = directory.to_string_lossy().to_string();
    proxy
        .start_cycling(dir_str, interval, mode_str.to_string())
        .await
        .map_err(|err| format!("Failed to start cycling: {err}"))?;

    println!(
        "Started cycling wallpapers from {} every {interval} seconds",
        directory.display(),
    );
    Ok(())
}
