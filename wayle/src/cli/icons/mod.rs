/// Icon command definitions
pub mod commands;
/// Install icons from CDN
pub mod install;
/// Open icons directory
pub mod open;
/// Remove installed icons
pub mod remove;
/// Install bundled icons
pub mod setup;

use commands::IconsCommands;

use super::CliAction;

/// Executes icon management commands.
///
/// # Errors
///
/// Returns error if the command execution fails.
pub async fn execute(command: IconsCommands) -> CliAction {
    match command {
        IconsCommands::Setup => setup::execute(),
        IconsCommands::Install { source, slugs } => install::execute(source, slugs).await,
        IconsCommands::Remove { names } => remove::execute(names),
        IconsCommands::Open => open::execute(),
    }
}
