//! Panel restart command.

use super::{proxy::is_running, start, stop};
use crate::cli::CliAction;

/// Restarts the Wayle GUI panel process.
///
/// Stops the running panel and waits for shutdown before starting a new instance.
///
/// # Errors
///
/// Returns error if panel cannot be stopped or started.
pub async fn execute() -> CliAction {
    if is_running().await.unwrap_or(false) {
        stop::execute().await?;
    }
    start::execute().await
}
