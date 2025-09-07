use super::utils::get_player_or_active;
use crate::{
    cli::CliAction,
    services::media::service::{Config, MediaService},
};

/// Execute the command
pub async fn execute(player: Option<String>) -> CliAction {
    let service = MediaService::new(Config {
        ignored_players: vec![],
    })
    .await
    .map_err(|e| format!("Failed to start media service: {e}"))?;

    let player = get_player_or_active(&service, player.as_ref()).await?;

    player
        .play_pause()
        .await
        .map_err(|e| format!("Failed to toggle play/pause: {e}"))?;

    Ok(())
}
