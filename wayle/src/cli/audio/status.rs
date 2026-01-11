use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the status command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    let volume = proxy
        .volume()
        .await
        .map_err(|e| format_error("get volume", e))?;

    let muted = proxy
        .muted()
        .await
        .map_err(|e| format_error("get mute state", e))?;

    let default_sink = proxy
        .default_sink()
        .await
        .map_err(|e| format_error("get default sink", e))?;

    let sink_count = proxy
        .sink_count()
        .await
        .map_err(|e| format_error("get sink count", e))?;

    let source_count = proxy
        .source_count()
        .await
        .map_err(|e| format_error("get source count", e))?;

    let mute_indicator = if muted { " (muted)" } else { "" };
    println!("Volume: {volume:.0}%{mute_indicator}");
    println!("Default output: {default_sink}");
    println!("Outputs: {sink_count}, Inputs: {source_count}");

    Ok(())
}
