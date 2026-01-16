//! Wayle CLI - Entry point for Wayle command-line interface.
//!
//! CLI commands for managing Wayle services.
//! The GUI panel is a separate binary (`wayle-shell`).

use std::process;

use clap::Parser;
use wayle::{
    cli::{Cli, Commands},
    core::{init, tracing as tracing_init},
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = tracing_init::init_cli_mode() {
        eprintln!("Failed to initialize tracing: {e}");
    }

    if let Err(e) = init::ensure_directories() {
        eprintln!("Failed to ensure directories: {e}");
    }

    let result = match cli.command {
        Commands::Audio { command } => wayle::cli::audio::execute(command).await,
        Commands::Config { command } => wayle::cli::config::execute(command).await,
        Commands::Icons { command } => wayle::cli::icons::execute(command).await,
        Commands::Media { command } => wayle::cli::media::execute(command).await,
        Commands::Notify { command } => wayle::cli::notify::execute(command).await,
        Commands::Panel { command } => wayle::cli::panel::execute(command).await,
        Commands::Power { command } => wayle::cli::power::execute(command).await,
        Commands::Systray { command } => wayle::cli::systray::execute(command).await,
        Commands::Wallpaper { command } => wayle::cli::wallpaper::execute(command).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
