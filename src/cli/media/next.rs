use super::utils::get_player_or_active;
use crate::{
    cli::CliAction,
    services::media::{Config, MediaService},
};

pub async fn execute(player: Option<String>) -> CliAction {
    let service = MediaService::start(Config {
        ignored_players: vec![],
    })
    .await
    .map_err(|e| format!("Failed to start media service: {e}"))?;

    let player = get_player_or_active(&service, player.as_ref()).await?;

    player
        .next()
        .await
        .map_err(|e| format!("Failed to skip to next track: {e}"))?;

    Ok(())
}
