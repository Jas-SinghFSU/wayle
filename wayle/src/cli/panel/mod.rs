/// Panel command definitions
pub mod commands;

use commands::PanelCommands;

use crate::cli::CliAction;

/// Executes panel management commands
///
/// # Errors
/// Returns error if the command execution fails.
/// Execute the command
pub async fn execute(command: commands::PanelCommands) -> CliAction {
    match command {
        PanelCommands::Start => commands::start().await,
        PanelCommands::Stop => commands::stop().await,
        PanelCommands::Restart => commands::restart().await,
        PanelCommands::Status => commands::status().await,
        PanelCommands::Settings => commands::settings().await,
    }
}
