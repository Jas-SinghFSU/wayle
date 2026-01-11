use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the status command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    let count = proxy
        .count()
        .await
        .map_err(|e| format_error("get notification count", e))?;

    let popup_count = proxy
        .popup_count()
        .await
        .map_err(|e| format_error("get popup count", e))?;

    let dnd = proxy
        .dnd()
        .await
        .map_err(|e| format_error("get DND state", e))?;

    let popup_duration = proxy
        .popup_duration()
        .await
        .map_err(|e| format_error("get popup duration", e))?;

    let dnd_status = if dnd { "enabled" } else { "disabled" };
    println!("Notifications: {count}");
    println!("Active popups: {popup_count}");
    println!("Do Not Disturb: {dnd_status}");
    println!("Popup duration: {popup_duration}ms");

    Ok(())
}
