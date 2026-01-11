use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the sinks list command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    let sinks = proxy
        .list_sinks()
        .await
        .map_err(|e| format_error("list sinks", e))?;

    let default = proxy
        .default_sink()
        .await
        .map_err(|e| format_error("get default sink", e))?;

    if sinks.is_empty() {
        println!("No audio sinks found");
        return Ok(());
    }

    println!("Audio outputs:");
    for (_index, name, description) in &sinks {
        let marker = if name == &default { " *" } else { "" };
        println!("  {description}{marker}");
    }

    Ok(())
}
