use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the cycle command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    proxy
        .cycle()
        .await
        .map_err(|e| format_error("cycle profile", e))?;

    let active = proxy
        .active_profile()
        .await
        .map_err(|e| format_error("get active profile", e))?;

    println!("Profile: {active}");

    Ok(())
}
