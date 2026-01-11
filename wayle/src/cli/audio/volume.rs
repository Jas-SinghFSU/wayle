use wayle_audio::dbus::AudioProxy;

use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the volume command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute(level: Option<String>) -> CliAction {
    let (_connection, proxy) = connect().await?;

    match level {
        Some(value) => set_volume(&proxy, &value).await,
        None => show_volume(&proxy).await,
    }
}

async fn show_volume(proxy: &AudioProxy<'_>) -> CliAction {
    let volume = proxy
        .volume()
        .await
        .map_err(|e| format_error("get volume", e))?;

    let muted = proxy
        .muted()
        .await
        .map_err(|e| format_error("get mute state", e))?;

    print_volume(volume, muted);
    Ok(())
}

fn print_volume(volume: f64, muted: bool) {
    if muted {
        println!("Volume: {volume:.0}% (muted)");
    } else {
        println!("Volume: {volume:.0}%");
    }
}

async fn set_volume(proxy: &AudioProxy<'_>, value: &str) -> CliAction {
    let muted = proxy
        .muted()
        .await
        .map_err(|e| format_error("get mute state", e))?;

    let new_volume = if let Some(delta_str) = value.strip_prefix('+') {
        let delta: f64 = delta_str
            .parse()
            .map_err(|_| format!("Invalid volume delta: {value}"))?;
        proxy
            .adjust_volume(delta)
            .await
            .map_err(|e| format_error("adjust volume", e))?
    } else if let Some(delta_str) = value.strip_prefix('-') {
        let delta: f64 = delta_str
            .parse()
            .map_err(|_| format!("Invalid volume delta: {value}"))?;
        proxy
            .adjust_volume(-delta)
            .await
            .map_err(|e| format_error("adjust volume", e))?
    } else {
        let volume: f64 = value
            .parse()
            .map_err(|_| format!("Invalid volume level: {value}"))?;
        proxy
            .set_volume(volume)
            .await
            .map_err(|e| format_error("set volume", e))?
    };

    print_volume(new_volume, muted);
    Ok(())
}
