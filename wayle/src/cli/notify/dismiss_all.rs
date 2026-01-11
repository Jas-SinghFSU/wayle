use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the dismiss-all command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    proxy
        .dismiss_all()
        .await
        .map_err(|e| format_error("dismiss notifications", e))?;

    println!("Dismissed all notifications");

    Ok(())
}
