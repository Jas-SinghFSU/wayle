/// Configuration command definitions
pub mod commands;
/// Get configuration value command
pub mod get;
/// Set configuration value command  
pub mod set;
/// Watch configuration changes command
pub mod watch;

use commands::ConfigCommands;

use super::CliAction;

/// Executes configuration management commands.
///
/// # Errors
/// Returns error if the command execution fails.
/// Execute the command
pub async fn execute(command: commands::ConfigCommands) -> CliAction {
    match command {
        ConfigCommands::Get { path } => get::execute(path).await,
        ConfigCommands::Set { path, value } => set::execute(path, value).await,
        ConfigCommands::Watch { path } => watch::execute(path).await,
    }
}
