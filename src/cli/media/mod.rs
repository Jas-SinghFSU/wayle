mod active;
mod commands;
mod info;
mod list;
mod loop_mode;
mod next;
mod play_pause;
mod previous;
mod seek;
mod shuffle;
mod utils;

pub use commands::MediaCommands;

use super::CliAction;

/// Executes media player control commands.
///
/// # Errors
/// Returns error if the command execution fails.
pub async fn execute(command: MediaCommands) -> CliAction {
    match command {
        MediaCommands::List => list::execute().await,
        MediaCommands::PlayPause { player } => play_pause::execute(player).await,
        MediaCommands::Next { player } => next::execute(player).await,
        MediaCommands::Previous { player } => previous::execute(player).await,
        MediaCommands::Seek { position, player } => seek::execute(position, player).await,
        MediaCommands::Shuffle { state, player } => shuffle::execute(state, player).await,
        MediaCommands::Loop { mode, player } => loop_mode::execute(mode, player).await,
        MediaCommands::Active { player } => active::execute(player).await,
        MediaCommands::Info { player } => info::execute(player).await,
    }
}
