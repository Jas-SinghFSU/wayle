//! Wallpaper control commands (stop, next, previous).

use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the stop command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails.
pub async fn stop() -> CliAction {
    let (_connection, proxy) = connect().await?;
    proxy
        .stop_cycling()
        .await
        .map_err(|e| format_error("stop cycling", e))?;

    println!("Wallpaper cycling stopped");
    Ok(())
}

/// Executes the next command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails.
pub async fn next() -> CliAction {
    let (_connection, proxy) = connect().await?;
    proxy
        .next()
        .await
        .map_err(|e| format_error("advance wallpaper", e))?;

    println!("Advanced to next wallpaper");
    Ok(())
}

/// Executes the previous command.
///
/// # Errors
///
/// Returns error if D-Bus connection fails.
pub async fn previous() -> CliAction {
    let (_connection, proxy) = connect().await?;
    proxy
        .previous()
        .await
        .map_err(|e| format_error("go to previous wallpaper", e))?;

    println!("Went back to previous wallpaper");
    Ok(())
}
