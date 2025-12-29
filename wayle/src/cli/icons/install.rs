use wayle_icons::{IconManager, sources};

use crate::cli::CliAction;

/// Installs icons from a CDN source.
///
/// # Errors
///
/// Returns error if:
/// - Source name is invalid
/// - Icon fetch fails
/// - File write fails
pub async fn execute(source_name: String, slugs: Vec<String>) -> CliAction {
    let source = sources::from_cli_name(&source_name).map_err(|err| err.to_string())?;

    let manager = IconManager::new().map_err(|err| err.to_string())?;

    let slug_refs: Vec<&str> = slugs.iter().map(String::as_str).collect();

    let installed = manager
        .install(source.as_ref(), &slug_refs)
        .await
        .map_err(|err| err.to_string())?;

    for name in &installed {
        println!("Installed: {name}");
    }

    Ok(())
}
