use std::{fs, path::Path};

use crate::config::error::Error;

/// Creates a default configuration file if it doesn't exist
pub fn create_default_config_file(path: &Path) -> Result<(), Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| Error::IoError {
            path: parent.to_path_buf(),
            details: format!("Failed to create config directory: {e}"),
        })?;
    }

    fs::write(path, "# Wayle configuration file\n").map_err(|e| Error::IoError {
        path: path.to_path_buf(),
        details: format!("Failed to create config file: {e}"),
    })?;

    Ok(())
}
