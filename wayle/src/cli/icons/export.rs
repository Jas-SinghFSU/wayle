use std::{fs, path::PathBuf};

use wayle_icons::IconManager;

use crate::cli::CliAction;

/// Exports all installed icons to a destination directory.
///
/// # Errors
///
/// Returns error if icon manager fails or file operations fail.
pub fn execute(destination: PathBuf) -> CliAction {
    let manager = IconManager::new().map_err(|err| err.to_string())?;
    let icons = manager.list();

    if icons.is_empty() {
        println!("No icons installed");
        return Ok(());
    }

    fs::create_dir_all(&destination)
        .map_err(|err| format!("cannot create {}: {err}", destination.display()))?;

    let source_dir = manager.registry().icons_dir();
    let mut copied = 0;

    for icon in &icons {
        let src = source_dir.join(format!("{icon}.svg"));
        let dest = destination.join(format!("{icon}.svg"));

        if let Err(err) = fs::copy(&src, &dest) {
            eprintln!("Failed to copy {icon}: {err}");
            continue;
        }
        copied += 1;
    }

    println!("Exported {copied} icons to {}", destination.display());

    Ok(())
}
