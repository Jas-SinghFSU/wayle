//! Wallpaper configuration commands.

use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the theming-monitor command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails.
pub async fn set_theming_monitor(monitor: String) -> CliAction {
    let (_connection, proxy) = connect().await?;
    proxy
        .set_theming_monitor(monitor.clone())
        .await
        .map_err(|e| format_error("set theming monitor", e))?;

    if monitor.is_empty() {
        println!("Theming monitor: default");
    } else {
        println!("Theming monitor: {monitor}");
    }
    Ok(())
}
