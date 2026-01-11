use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the sources list command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    let sources = proxy
        .list_sources()
        .await
        .map_err(|e| format_error("list sources", e))?;

    let default = proxy
        .default_source()
        .await
        .map_err(|e| format_error("get default source", e))?;

    if sources.is_empty() {
        println!("No audio sources found");
        return Ok(());
    }

    println!("Audio inputs:");
    for (_index, name, description) in &sources {
        let marker = if name == &default { " *" } else { "" };
        println!("  {description}{marker}");
    }

    Ok(())
}
