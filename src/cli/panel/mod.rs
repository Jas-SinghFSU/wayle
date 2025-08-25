mod commands;

pub use commands::PanelCommands;

use crate::cli::CliAction;

/// Executes panel management commands
///
/// # Errors
/// Returns error if the command execution fails.
pub async fn execute(command: PanelCommands) -> CliAction {
    match command {
        PanelCommands::Start => commands::start().await,
        PanelCommands::Stop => commands::stop().await,
        PanelCommands::Restart => commands::restart().await,
        PanelCommands::Status => commands::status().await,
        PanelCommands::Settings => commands::settings().await,
    }
}
