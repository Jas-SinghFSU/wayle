use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the mute toggle command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    let muted = proxy
        .toggle_mute()
        .await
        .map_err(|e| format_error("toggle mute", e))?;

    if muted {
        println!("Muted");
    } else {
        println!("Unmuted");
    }

    Ok(())
}
