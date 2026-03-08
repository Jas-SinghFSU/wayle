use std::fs;

use wayle_config::{Config, ConfigPaths};

use crate::cli::CliAction;

/// Outputs the default configuration as TOML.
///
/// Serializes `Config::default()` so the output always matches the current schema.
/// Use `stdout` flag to print to terminal, otherwise writes `config.toml.example`
/// to the config directory.
///
/// # Errors
///
/// Returns error if serialization or file write fails.
pub fn execute(stdout: bool) -> CliAction {
    let toml = toml::to_string_pretty(&Config::default())
        .map_err(|e| format!("Failed to serialize default config: {e}"))?;

    if stdout {
        println!("{toml}");
        return Ok(());
    }

    let path = ConfigPaths::example_config();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {e}"))?;
    }

    fs::write(&path, &toml).map_err(|e| format!("Failed to write example config: {e}"))?;

    println!("Written:");
    println!("  {}", path.display());

    Ok(())
}
