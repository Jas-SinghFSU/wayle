use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the status command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    let active = proxy
        .active_profile()
        .await
        .map_err(|e| format_error("get active profile", e))?;

    let degraded = proxy
        .performance_degraded()
        .await
        .map_err(|e| format_error("get degradation status", e))?;

    println!("Active profile: {active}");
    if !degraded.is_empty() {
        println!("Performance degraded: {degraded}");
    }

    Ok(())
}
