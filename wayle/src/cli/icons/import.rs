use std::path::PathBuf;

use wayle_icons::IconManager;

use crate::cli::CliAction;

/// Imports a local SVG file as a custom icon.
///
/// # Errors
///
/// Returns error if icon manager initialization or import fails.
pub fn execute(path: PathBuf, name: String) -> CliAction {
    let manager = IconManager::new().map_err(|err| err.to_string())?;
    let icon_name = manager
        .import_local(&path, &name)
        .map_err(|err| err.to_string())?;

    println!("Installed: {icon_name}");
    Ok(())
}
