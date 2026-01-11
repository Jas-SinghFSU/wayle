use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the dnd toggle command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    proxy
        .toggle_dnd()
        .await
        .map_err(|e| format_error("toggle DND", e))?;

    let dnd = proxy
        .dnd()
        .await
        .map_err(|e| format_error("get DND state", e))?;

    if dnd {
        println!("Do Not Disturb: enabled");
    } else {
        println!("Do Not Disturb: disabled");
    }

    Ok(())
}
