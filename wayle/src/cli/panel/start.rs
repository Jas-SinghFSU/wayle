//! Panel start command.

use std::{
    io::ErrorKind,
    process::{Command, Stdio},
};

use tracing::info;

use super::proxy::is_running;
use crate::cli::CliAction;

/// Starts the Wayle GUI panel process.
///
/// Spawns the `wayle-shell` binary as a detached daemon process.
/// If the panel is already running, reports that and returns success.
///
/// # Errors
///
/// Returns error if wayle-shell cannot be found or executed.
pub async fn execute() -> CliAction {
    if is_running().await.unwrap_or(false) {
        println!("Panel is already running");
        return Ok(());
    }

    info!("Starting Wayle panel");

    Command::new("wayle-shell")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => {
                "wayle-shell not found. Is Wayle installed correctly?".to_string()
            }
            ErrorKind::PermissionDenied => {
                "Permission denied when starting wayle-shell".to_string()
            }
            _ => format!("Failed to start panel: {e}"),
        })?;

    println!("Panel started");
    Ok(())
}
