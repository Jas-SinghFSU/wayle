/// Notification command definitions
pub mod commands;
/// Dismiss notification command
pub mod dismiss;
/// Dismiss all notifications command
pub mod dismiss_all;
/// Do Not Disturb toggle command
pub mod dnd;
/// List notifications command
pub mod list;
mod proxy;
/// Status command
pub mod status;

use commands::NotifyCommands;

use super::CliAction;

/// Executes notification control commands.
///
/// # Errors
/// Returns error if the command execution fails.
pub async fn execute(command: NotifyCommands) -> CliAction {
    match command {
        NotifyCommands::List => list::execute().await,
        NotifyCommands::Dismiss { id } => dismiss::execute(id).await,
        NotifyCommands::DismissAll => dismiss_all::execute().await,
        NotifyCommands::Dnd => dnd::execute().await,
        NotifyCommands::Status => status::execute().await,
    }
}
