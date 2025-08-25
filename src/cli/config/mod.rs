mod commands;
mod get;
mod set;
mod watch;

pub use commands::ConfigCommands;

use super::CliAction;

/// Executes configuration management commands.
///
/// # Errors
/// Returns error if the command execution fails.
pub async fn execute(command: ConfigCommands) -> CliAction {
    match command {
        ConfigCommands::Get { path } => get::execute(path).await,
        ConfigCommands::Set { path, value } => set::execute(path, value).await,
        ConfigCommands::Watch { path } => watch::execute(path).await,
    }
}
