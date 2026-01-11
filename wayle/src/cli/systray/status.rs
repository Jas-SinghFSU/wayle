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
        .map_err(|e| format_error("get item count", e))?;

    let is_watcher = proxy
        .is_watcher()
        .await
        .map_err(|e| format_error("get watcher status", e))?;

    let watcher_status = if is_watcher { "active" } else { "inactive" };
    println!("Tray items: {count}");
    println!("StatusNotifierWatcher: {watcher_status}");

    Ok(())
}
