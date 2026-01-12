//! Panel settings command.

use std::{
    io::ErrorKind,
    process::{Command, Stdio},
};

use tracing::info;

use crate::cli::CliAction;

/// Launches the settings application.
///
/// # Errors
///
/// Returns error if settings application cannot be launched.
pub async fn execute() -> CliAction {
    info!("Launching Wayle settings");

    Command::new("wayle-settings")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => {
                "wayle-settings not found. Is Wayle installed correctly?".to_string()
            }
            ErrorKind::PermissionDenied => {
                "Permission denied when starting wayle-settings".to_string()
            }
            _ => format!("Failed to launch settings: {e}"),
        })?;

    Ok(())
}
