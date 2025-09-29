use wayle_media::{MediaService, types::LoopMode};

use super::{commands::LoopModeArg, utils::get_player_or_active};
use crate::cli::CliAction;

/// Execute the command
///
/// # Errors
/// Returns error if service communication fails or player is not found.
pub async fn execute(mode: LoopModeArg, player: Option<String>) -> CliAction {
    let service = MediaService::new()
        .await
        .map_err(|e| format!("Failed to connect to media service: {e}"))?;

    let player = get_player_or_active(&service, player.as_ref()).await?;

    let loop_mode = match mode {
        LoopModeArg::None => LoopMode::None,
        LoopModeArg::Track => LoopMode::Track,
        LoopModeArg::Playlist => LoopMode::Playlist,
    };

    player
        .set_loop_mode(loop_mode)
        .await
        .map_err(|e| format!("Failed to set loop mode: {e}"))?;

    println!("Loop mode set to {loop_mode:?}");
    Ok(())
}
