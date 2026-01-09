use wayle_wallpaper::WallpaperProxy;
use zbus::Connection;

use crate::cli::CliAction;

/// Executes the info command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails.
pub async fn execute(monitor: Option<String>) -> CliAction {
    let connection = Connection::session()
        .await
        .map_err(|err| format!("Failed to connect to D-Bus: {err}"))?;

    let proxy = WallpaperProxy::new(&connection)
        .await
        .map_err(|err| format!("Failed to connect to wallpaper service: {err}"))?;

    let monitor_arg = monitor.clone().unwrap_or_default();

    let wallpaper = proxy
        .wallpaper_for_monitor(monitor_arg)
        .await
        .map_err(|err| format!("Failed to get wallpaper: {err}"))?;

    let fit_mode = proxy
        .get_fit_mode()
        .await
        .map_err(|err| format!("Failed to get fit mode: {err}"))?;

    let cycling = proxy
        .get_is_cycling()
        .await
        .map_err(|err| format!("Failed to get cycling state: {err}"))?;

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
