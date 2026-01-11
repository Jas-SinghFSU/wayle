use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the activate command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute(id: String) -> CliAction {
    let (_connection, proxy) = connect().await?;

    proxy
        .activate(id.clone())
        .await
        .map_err(|e| format_error("activate tray item", e))?;

    println!("Activated: {id}");

    Ok(())
}
