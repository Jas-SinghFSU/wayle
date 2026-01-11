use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the info command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails.
pub async fn execute(monitor: Option<String>) -> CliAction {
    let (_connection, proxy) = connect().await?;

    let monitor_arg = monitor.clone().unwrap_or_default();

    let wallpaper = proxy
        .wallpaper_for_monitor(monitor_arg)
        .await
        .map_err(|e| format_error("get wallpaper", e))?;

    let fit_mode = proxy
        .get_fit_mode()
        .await
        .map_err(|e| format_error("get fit mode", e))?;

    let cycling = proxy
        .get_is_cycling()
        .await
        .map_err(|e| format_error("get cycling state", e))?;

    match &monitor {
        Some(mon) => {
            println!("Wallpaper Information ({mon})");
            println!("-----------------------------");
        }
        None => {
            println!("Wallpaper Information");
            println!("---------------------");
        }
    }

    if wallpaper.is_empty() {
        println!("Current:    (none)");
    } else {
        println!("Current:    {wallpaper}");
    }

    println!("Fit Mode:   {fit_mode}");
    println!(
        "Cycling:    {}",
        if cycling { "active" } else { "inactive" }
    );

    Ok(())
}
