//! Panel management commands.

/// Panel command definitions.
pub mod commands;
mod proxy;
/// Restart command.
pub mod restart;
/// Settings command.
pub mod settings;
/// Start command.
pub mod start;
/// Status command.
pub mod status;
/// Stop command.
pub mod stop;

use commands::PanelCommands;

use crate::cli::CliAction;

/// Executes panel management commands.
///
/// # Errors
///
/// Returns error if the command execution fails.
pub async fn execute(command: PanelCommands) -> CliAction {
    match command {
        PanelCommands::Start => start::execute().await,
        PanelCommands::Stop => stop::execute().await,
        PanelCommands::Restart => restart::execute().await,
        PanelCommands::Status => status::execute().await,
        PanelCommands::Settings => settings::execute().await,
    }
}
