use std::{fs, path::Path};

use wayle_icons::IconRegistry;

use crate::cli::CliAction;

const RESOURCES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../resources/icons");

/// Installs bundled icons from the resources directory.
///
/// # Errors
///
/// Returns error if source directory doesn't exist or copy fails.
pub fn execute() -> CliAction {
    let source_dir = Path::new(RESOURCES_DIR);

    if !source_dir.exists() {
        return Err(format!(
            "Resources directory not found: {}",
            source_dir.display()
        ));
    }

    let registry = IconRegistry::new().map_err(|err| err.to_string())?;
    let dest_dir = registry.icons_dir();

    fs::create_dir_all(&dest_dir)
        .map_err(|err| format!("Failed to create icons directory: {err}"))?;

    let entries = fs::read_dir(source_dir)
        .map_err(|err| format!("Failed to read resources directory: {err}"))?;

    let mut count = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(filename) = path.file_name() else {
            continue;
        };
        if path.extension().is_some_and(|ext| ext == "svg") {
            let dest_path = dest_dir.join(filename);
            fs::copy(&path, &dest_path)
                .map_err(|err| format!("Failed to copy {}: {err}", path.display()))?;
            println!(
                "Installed: {}",
                filename.to_string_lossy().trim_end_matches(".svg")
            );
            count += 1;
        }
    }

    println!("\n{count} icons installed to {}", dest_dir.display());
    Ok(())
}
