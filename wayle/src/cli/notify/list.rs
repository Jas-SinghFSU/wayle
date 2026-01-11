use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the list command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    let notifications = proxy
        .list()
        .await
        .map_err(|e| format_error("list notifications", e))?;

    if notifications.is_empty() {
        println!("No notifications");
        return Ok(());
    }

    println!("Notifications:");
    for (id, app_name, summary, body) in &notifications {
        println!("  [{id}] {app_name}: {summary}");
        if !body.is_empty() {
            println!("      {body}");
        }
    }

    Ok(())
}
