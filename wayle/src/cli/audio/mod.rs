/// Audio command definitions.
pub mod commands;
mod input_mute;
mod input_volume;
mod output_mute;
mod output_volume;
mod proxy;
/// Sinks list command.
pub mod sinks;
/// Sources list command.
pub mod sources;
/// Status command.
pub mod status;

use commands::AudioCommands;

use super::CliAction;

/// Executes audio control commands.
///
/// # Errors
/// Returns error if the command execution fails.
pub async fn execute(command: AudioCommands) -> CliAction {
    match command {
        AudioCommands::OutputVolume { level } => output_volume::execute(level).await,
        AudioCommands::OutputMute => output_mute::execute().await,
        AudioCommands::InputVolume { level } => input_volume::execute(level).await,
        AudioCommands::InputMute => input_mute::execute().await,
        AudioCommands::Sinks => sinks::execute().await,
        AudioCommands::Sources => sources::execute().await,
        AudioCommands::Status => status::execute().await,
    }
}
