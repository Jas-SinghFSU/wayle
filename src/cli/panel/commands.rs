use clap::Subcommand;

use crate::cli::CliAction;

/// Panel management subcommands.
#[derive(Subcommand, Debug)]
pub enum PanelCommands {
    /// Start the Wayle panel
    Start,
    /// Stop the Wayle panel
    Stop,
    /// Restart the Wayle panel
    Restart,
    /// Check panel status
    Status,
    /// Open panel settings
    Settings,
}

/// Starts the Wayle GUI panel process
///
/// # Errors
/// Returns error if panel process cannot be started.
pub async fn start() -> CliAction {
    todo!("Implement panel start")
}

/// Stops the Wayle GUI panel process
///
/// # Errors
/// Returns error if panel process cannot be stopped.
pub async fn stop() -> CliAction {
    todo!("Implement panel stop")
}

/// Restarts the Wayle GUI panel process
///
/// # Errors
/// Returns error if panel cannot be stopped or started.
pub async fn restart() -> CliAction {
    stop().await?;
    start().await
}

/// Checks the status of the Wayle GUI panel process
///
/// # Errors
/// Returns error if panel status cannot be determined.
pub async fn status() -> CliAction {
    todo!("Implement panel status check")
}

/// Launches the panel settings GUI
///
/// # Errors
/// Returns error if settings GUI cannot be launched.
pub async fn settings() -> CliAction {
    todo!("Implement panel settings GUI launch")
}
