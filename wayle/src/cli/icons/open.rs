use std::process::Command;

use wayle_icons::IconRegistry;

use crate::cli::CliAction;

/// Opens the icons directory in the default file manager.
///
/// Initializes the registry (creates directory structure and index.theme)
/// if it doesn't exist yet.
///
/// # Errors
///
/// Returns error if registry initialization or xdg-open fails.
pub fn execute() -> CliAction {
    let registry = IconRegistry::new().map_err(|err| err.to_string())?;
    registry.ensure_setup().map_err(|err| err.to_string())?;

    let path = registry.base_path();

    Command::new("xdg-open")
        .arg(path)
        .spawn()
        .map_err(|err| format!("Failed to run xdg-open: {err}"))?;

    println!("Opened: {}", path.display());
    Ok(())
}
