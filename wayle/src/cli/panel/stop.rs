//! Panel stop command.

use std::collections::HashMap;

use tracing::info;
use wayle_common::shell::actions;

use super::proxy::{actions_proxy, connect, is_running, wait_for_shutdown};
use crate::cli::CliAction;

/// Stops the Wayle GUI panel process via D-Bus.
///
/// Sends a quit action to the running GApplication instance and waits
/// for it to release its D-Bus name.
///
/// # Errors
///
/// Returns error if panel is not running or cannot be stopped.
pub async fn execute() -> CliAction {
    if !is_running().await.unwrap_or(false) {
        return Err("Panel is not running".to_string());
    }

    info!("Stopping Wayle panel");

    let connection = connect().await?;
    let proxy = actions_proxy(&connection).await?;

    proxy
        .activate(actions::QUIT, Vec::new(), HashMap::new())
        .await
        .map_err(|e| format!("Failed to stop panel: {e}"))?;

    wait_for_shutdown(&connection).await?;

    println!("Panel stopped");
    Ok(())
}
