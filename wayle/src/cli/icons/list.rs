use std::{
    io::Write,
    process::{Command, Stdio},
};

use wayle_icons::IconManager;

use crate::cli::CliAction;

/// Lists installed icons, optionally filtered by source prefix.
///
/// When interactive mode is enabled, pipes the list through fzf for fuzzy search.
///
/// # Errors
///
/// Returns error if icon manager initialization fails or fzf is not available
/// when interactive mode is requested.
pub fn execute(source_filter: Option<String>, interactive: bool) -> CliAction {
    let manager = IconManager::new().map_err(|err| err.to_string())?;
    let mut icons = manager.list();

    if let Some(prefix) = &source_filter {
        let filter_prefix = format!("{prefix}-");
        icons.retain(|name| name.starts_with(&filter_prefix));
    }

    if icons.is_empty() {
        match source_filter {
            Some(prefix) => println!("No icons installed with prefix '{prefix}-'"),
            None => println!("No icons installed"),
        }
        return Ok(());
    }

    icons.sort();

    if interactive {
        return run_fzf(&icons);
    }

    println!("\nInstalled icons ({}):\n", icons.len());

    for icon in icons {
        println!("  {icon}");
    }

    println!();

    Ok(())
}

/// Runs fzf with the icon list for interactive fuzzy search.
///
/// Selected icon name is copied to clipboard.
fn run_fzf(icons: &[String]) -> CliAction {
    let mut child = Command::new("fzf")
        .args(["--prompt", "Search icons: "])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| "fzf not found. Install fzf or use without -i flag.".to_string())?;

    if let Some(stdin) = child.stdin.as_mut() {
        for icon in icons {
            let _ = writeln!(stdin, "{icon}");
        }
    }

    let output = child.wait_with_output().map_err(|err| err.to_string())?;

    if !output.status.success() {
        return Ok(());
    }

    let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if selected.is_empty() {
        return Ok(());
    }

    match copy_to_clipboard(&selected) {
        Ok(()) => println!("Copied to clipboard: {selected}"),
        Err(err) => eprintln!("Failed to copy to clipboard: {err}"),
    }

    Ok(())
}

/// Copies text to clipboard using wl-copy.
fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut child = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| "wl-copy not found. Install wl-clipboard.".to_string())?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(text.as_bytes())
            .map_err(|err| format!("Failed to write to wl-copy: {err}"))?;
    }

    let output = child.wait_with_output().map_err(|err| err.to_string())?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("wl-copy failed: {stderr}"));
    }

    Ok(())
}
