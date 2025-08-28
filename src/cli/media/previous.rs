use super::utils::get_player_or_active;
use crate::{
    cli::CliAction,
    services::media::{Config, MediaService},
};

pub async fn execute(player: Option<String>) -> CliAction {
    let service = MediaService::new(Config {
        ignored_players: vec![],
    })
    .await
    .map_err(|e| format!("Failed to start media service: {e}"))?;

    let player = get_player_or_active(&service, player.as_ref()).await?;

    player
        .previous()
        .await
        .map_err(|e| format!("Failed to go to previous track: {e}"))?;

    Ok(())
}
