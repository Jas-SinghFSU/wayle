/// Application CLI parser and command definitions.
pub mod app;
/// Configuration management CLI commands.
pub mod config;
/// Media player control CLI commands.
pub mod media;
/// Panel management CLI commands.
pub mod panel;

pub use app::{Cli, Commands};

/// Result type for CLI operations that return output.
pub type CliResult = Result<String, String>;
/// Result type for CLI operations that perform actions.
pub type CliAction = Result<(), String>;
