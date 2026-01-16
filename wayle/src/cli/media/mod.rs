/// Active player command
pub mod active;
/// Media command definitions
pub mod commands;
/// Player info command
pub mod info;
/// List players command
pub mod list;
/// Loop mode command
pub mod loop_mode;
/// Next track command
pub mod next;
/// Play/pause command
pub mod play_pause;
/// Previous track command
pub mod previous;
mod proxy;
mod resolve;
/// Shuffle command
pub mod shuffle;

use commands::MediaCommands;

use super::CliAction;

/// Executes media player control commands.
///
/// # Errors
/// Returns error if the command execution fails.
pub async fn execute(command: commands::MediaCommands) -> CliAction {
    match command {
        MediaCommands::List => list::execute().await,
        MediaCommands::PlayPause { player } => play_pause::execute(player).await,
        MediaCommands::Next { player } => next::execute(player).await,
        MediaCommands::Previous { player } => previous::execute(player).await,
        MediaCommands::Shuffle { state, player } => shuffle::execute(state, player).await,
        MediaCommands::Loop { mode, player } => loop_mode::execute(mode, player).await,
        MediaCommands::Active { player } => active::execute(player).await,
        MediaCommands::Info { player } => info::execute(player).await,
    }
}
