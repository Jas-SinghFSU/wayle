//! Panel inspect command.

use std::collections::HashMap;

use wayle_common::shell::actions;

use super::proxy::{actions_proxy, connect, is_running};
use crate::cli::CliAction;

/// Opens the GTK Inspector on the running panel via D-Bus.
///
/// # Errors
///
/// Returns error if panel is not running or action fails.
pub async fn execute() -> CliAction {
    if !is_running().await.unwrap_or(false) {
        return Err("Panel is not running".to_string());
    }

    let connection = connect().await?;
    let proxy = actions_proxy(&connection).await?;

    proxy
        .activate(actions::INSPECTOR, Vec::new(), HashMap::new())
        .await
        .map_err(|e| format!("Failed to open inspector: {e}"))?;

    println!("GTK Inspector opened");
    Ok(())
}
