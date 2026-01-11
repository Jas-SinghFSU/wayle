use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the list command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    let items = proxy
        .list()
        .await
        .map_err(|e| format_error("list tray items", e))?;

    if items.is_empty() {
        println!("No system tray items");
        return Ok(());
    }

    println!("System tray items:");
    for (id, title, icon_name, status) in &items {
        let display_name = if title.is_empty() { id } else { title };
        let icon_info = if icon_name.is_empty() {
            String::new()
        } else {
            format!(" ({icon_name})")
        };
        println!("  {display_name}{icon_info} [{status}]");
    }

    Ok(())
}
