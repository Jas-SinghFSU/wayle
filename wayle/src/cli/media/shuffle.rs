use super::{
    commands::ShuffleModeArg,
    proxy::{connect, format_error},
    resolve::resolve_player,
};
use crate::cli::CliAction;

/// Execute the command
///
/// # Errors
/// Returns error if D-Bus communication fails or player is not found.
pub async fn execute(state: Option<ShuffleModeArg>, player: Option<String>) -> CliAction {
    let (_connection, proxy) = connect().await?;

    let resolved = resolve_player(&proxy, player).await?;

    let state_str = match state {
        Some(ShuffleModeArg::On) => "on",
        Some(ShuffleModeArg::Off) => "off",
        Some(ShuffleModeArg::Toggle) | None => "toggle",
    };

    proxy
        .set_shuffle(resolved, state_str.to_string())
        .await
        .map_err(|e| format_error("set shuffle", e))?;

    Ok(())
}
