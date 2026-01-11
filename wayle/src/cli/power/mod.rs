/// Power profile command definitions
pub mod commands;
/// Cycle power profile command
pub mod cycle;
/// List power profiles command
pub mod list;
/// D-Bus proxy utilities
mod proxy;
/// Set power profile command
pub mod set;
/// Status command
pub mod status;

use commands::PowerCommands;

use super::CliAction;

/// Executes power profile control commands.
///
/// # Errors
/// Returns error if the command execution fails.
pub async fn execute(command: PowerCommands) -> CliAction {
    match command {
        PowerCommands::Status => status::execute().await,
        PowerCommands::Set { profile } => set::execute(profile).await,
        PowerCommands::Cycle => cycle::execute().await,
        PowerCommands::List => list::execute().await,
    }
}
