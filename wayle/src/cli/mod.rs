/// CLI application structure and parsing
pub mod app;
/// Configuration management commands
pub mod config;
/// Media control commands
pub mod media;
/// Panel management commands
pub mod panel;

/// Result type for CLI operations that return output text
pub type CliResult = Result<String, String>;
/// Result type for CLI operations that perform actions
pub type CliAction = Result<(), String>;

pub use app::{Cli, Commands};
