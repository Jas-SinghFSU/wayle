use std::path::PathBuf;

use super::{
    commands::CyclingModeArg,
    proxy::{connect, format_error},
};
use crate::cli::CliAction;

/// Executes the cycle command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails or directory is invalid.
pub async fn execute(directory: PathBuf, interval: u32, mode: CyclingModeArg) -> CliAction {
    let (_connection, proxy) = connect().await?;

    let mode_str = match mode {
        CyclingModeArg::Sequential => "sequential",
        CyclingModeArg::Shuffle => "shuffle",
    };

    let dir_str = directory.to_string_lossy().to_string();
    proxy
        .start_cycling(dir_str, interval, mode_str.to_string())
        .await
        .map_err(|e| format_error("start cycling", e))?;

    println!(
        "Started cycling wallpapers from {} every {interval} seconds",
        directory.display(),
    );
    Ok(())
}
