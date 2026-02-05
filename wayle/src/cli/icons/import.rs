use std::path::PathBuf;

use wayle_icons::IconManager;

use crate::cli::CliAction;

/// Imports local SVG file(s) as icons.
///
/// # Errors
///
/// Returns error if icon manager initialization or import fails.
pub fn execute(path: PathBuf, name: Option<String>) -> CliAction {
    let manager = IconManager::new().map_err(|err| err.to_string())?;

    if path.is_dir() {
        let result = manager.import_dir(&path).map_err(|err| err.to_string())?;

        if result.installed.is_empty() && result.failed.is_empty() {
            println!("No SVG files found in {}", path.display());
            return Ok(());
        }

        for name in &result.installed {
            println!("Imported: {name}");
        }

        for failure in &result.failed {
            eprintln!("Failed {}: {}", failure.slug, failure.error);
        }

        println!(
            "\nImported {} icons ({} failed)",
            result.installed.len(),
            result.failed.len()
        );

        if result.all_failed() {
            return Err("All imports failed".to_string());
        }

        return Ok(());
    }

    let Some(name) = name else {
        return Err("Name required when importing a single file".to_string());
    };

    let icon_name = manager
        .import_local(&path, &name)
        .map_err(|err| err.to_string())?;

    println!("Imported: {icon_name}");
    Ok(())
}
