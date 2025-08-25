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
pub async fn start() -> CliAction {
    todo!("Implement panel start")
}

/// Stops the Wayle GUI panel process
pub async fn stop() -> CliAction {
    todo!("Implement panel stop")
}

/// Restarts the Wayle GUI panel process
pub async fn restart() -> CliAction {
    stop().await?;
    start().await
}

/// Checks the status of the Wayle GUI panel process
pub async fn status() -> CliAction {
    todo!("Implement panel status check")
}

/// Launches the panel settings GUI
pub async fn settings() -> CliAction {
    todo!("Implement panel settings GUI launch")
}
