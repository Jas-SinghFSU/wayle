use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the input mute toggle command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    let muted = proxy
        .toggle_input_mute()
        .await
        .map_err(|e| format_error("toggle input mute", e))?;

    if muted {
        println!("Input muted");
    } else {
        println!("Input unmuted");
    }

    Ok(())
}
