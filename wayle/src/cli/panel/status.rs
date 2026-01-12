//! Panel status command.

use super::proxy::is_running;
use crate::cli::CliAction;

/// Reports the status of the Wayle GUI panel process.
///
/// # Errors
///
/// Returns error if status cannot be determined.
pub async fn execute() -> CliAction {
    match is_running().await {
        Ok(true) => {
            println!("Panel is running");
            Ok(())
        }
        Ok(false) => {
            println!("Panel is not running");
            Ok(())
        }
        Err(e) => Err(format!("Cannot determine panel status: {e}")),
    }
}
