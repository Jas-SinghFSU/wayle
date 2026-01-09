/// Wallpaper command definitions
pub mod commands;
/// Configuration commands (theming-monitor)
pub mod config;
/// Control commands (stop, next, previous)
pub mod control;
/// Cycling command
pub mod cycle;
/// Info command
pub mod info;
/// Set wallpaper command
pub mod set;

use commands::WallpaperCommands;

use super::CliAction;

/// Executes wallpaper commands.
///
/// # Errors
///
/// Returns error if the command execution fails.
pub async fn execute(command: WallpaperCommands) -> CliAction {
    match command {
        WallpaperCommands::Set { path, fit, monitor } => set::execute(path, fit, monitor).await,
        WallpaperCommands::Cycle {
            directory,
            interval,
            mode,
        } => cycle::execute(directory, interval, mode).await,
        WallpaperCommands::Stop => control::stop().await,
        WallpaperCommands::Next => control::next().await,
        WallpaperCommands::Previous => control::previous().await,
        WallpaperCommands::Info { monitor } => info::execute(monitor).await,
        WallpaperCommands::ThemingMonitor { monitor } => config::set_theming_monitor(monitor).await,
    }
}
