use wayle_icons::IconManager;

use crate::cli::CliAction;

/// Removes installed icons by name.
///
/// # Errors
///
/// Returns error if icon doesn't exist or deletion fails.
pub fn execute(names: Vec<String>) -> CliAction {
    let manager = IconManager::new().map_err(|err| err.to_string())?;

    for name in &names {
        manager.remove(name).map_err(|err| err.to_string())?;
        println!("Removed: {name}");
    }

    Ok(())
}
