use super::{
    commands::LoopModeArg,
    proxy::{connect, format_error},
    resolve::resolve_player,
};
use crate::cli::CliAction;

/// Execute the command
///
/// # Errors
/// Returns error if D-Bus communication fails or player is not found.
pub async fn execute(mode: LoopModeArg, player: Option<String>) -> CliAction {
    let (_connection, proxy) = connect().await?;

    let resolved = resolve_player(&proxy, player).await?;

    let mode_str = match mode {
        LoopModeArg::None => "none",
        LoopModeArg::Track => "track",
        LoopModeArg::Playlist => "playlist",
    };

    proxy
        .set_loop_status(resolved, mode_str.to_string())
        .await
        .map_err(|e| format_error("set loop mode", e))?;

    println!("Loop mode set to {mode_str}");
    Ok(())
}
