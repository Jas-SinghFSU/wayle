/// Idle inhibit command definitions.
pub mod commands;
mod duration;
mod off;
mod on;
mod proxy;
mod remaining;
/// Status command implementation.
pub mod status;
mod toggle;

use commands::IdleCommands;

use super::CliAction;

/// Executes idle inhibit control commands.
///
/// # Errors
/// Returns error if the command execution fails.
pub async fn execute(command: IdleCommands) -> CliAction {
    match command {
        IdleCommands::On {
            minutes,
            indefinite,
        } => on::execute(minutes, indefinite).await,
        IdleCommands::Off => off::execute().await,
        IdleCommands::Duration { value } => duration::execute(value).await,
        IdleCommands::Remaining { value } => remaining::execute(value).await,
        IdleCommands::Status => status::execute().await,
        IdleCommands::Toggle { indefinite } => toggle::execute(indefinite).await,
    }
}
