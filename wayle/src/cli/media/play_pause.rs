use super::{
    proxy::{connect, format_error},
    resolve::resolve_player,
};
use crate::cli::CliAction;

/// Execute the command
///
/// # Errors
/// Returns error if D-Bus communication fails or player is not found.
pub async fn execute(player: Option<String>) -> CliAction {
    let (_connection, proxy) = connect().await?;

    let resolved = resolve_player(&proxy, player).await?;

    proxy
        .play_pause(resolved)
        .await
        .map_err(|e| format_error("toggle play/pause", e))?;

    Ok(())
}
