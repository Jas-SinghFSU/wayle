use std::{error::Error, fs};

use tracing::{info, instrument};

use crate::config::ConfigPaths;

/// Ensures all required Wayle directories exist, creating them if necessary
///
/// # Errors
/// Returns error if directory creation fails.
#[instrument]
pub fn ensure_directories() -> Result<(), Box<dyn Error>> {
    let config_dir = ConfigPaths::config_dir()?;
    if !config_dir.exists() {
        info!("Creating config directory: {}", config_dir.display());
        fs::create_dir_all(&config_dir)?;
    }
    Ok(())
}
