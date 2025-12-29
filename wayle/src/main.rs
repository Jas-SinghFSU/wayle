//! Wayle orchestrator - Main entry point that manages panel and settings processes
//!
//! This binary is designed to always start successfully, even if dependencies are missing,
//! so it can provide diagnostic information to help users resolve issues.

use std::{error::Error, process};

use ::tracing::error;
use clap::Parser;
use wayle::{
    cli::{Cli, Commands},
    core::{init, tracing},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    tracing::init_cli_mode()?;
    init::ensure_directories()?;

    let result = match cli.command {
        Commands::Panel { command } => wayle::cli::panel::execute(command).await,
        Commands::Config { command } => wayle::cli::config::execute(command).await,
        Commands::Icons { command } => wayle::cli::icons::execute(command).await,
        Commands::Media { command } => wayle::cli::media::execute(command).await,
    };

    if let Err(e) = result {
        error!("Error: {e}");
        process::exit(1);
    }

    Ok(())
}
