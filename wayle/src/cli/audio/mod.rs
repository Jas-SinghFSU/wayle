/// Audio command definitions
pub mod commands;
/// Mute toggle command
pub mod mute;
mod proxy;
/// Sinks list command
pub mod sinks;
/// Sources list command
pub mod sources;
/// Status command
pub mod status;
/// Volume control command
pub mod volume;

use commands::AudioCommands;

use super::CliAction;

/// Executes audio control commands.
///
/// # Errors
/// Returns error if the command execution fails.
pub async fn execute(command: AudioCommands) -> CliAction {
    match command {
        AudioCommands::Volume { level } => volume::execute(level).await,
        AudioCommands::Mute => mute::execute().await,
        AudioCommands::Sinks => sinks::execute().await,
        AudioCommands::Sources => sources::execute().await,
        AudioCommands::Status => status::execute().await,
    }
}
