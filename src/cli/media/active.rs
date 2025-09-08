use super::utils::find_player_by_identifier;
use crate::{
    cli::CliAction,
    core::state::RuntimeState,
    services::media::{
        MediaService,
        types::{Config, PlaybackState},
    },
};

/// Execute the command
///
/// # Errors
/// Returns error if service communication fails or player is not found.
pub async fn execute(player: Option<String>) -> CliAction {
    let service = MediaService::new(Config {
        ignored_players: vec![],
    })
    .await
    .map_err(|e| format!("Failed to start media service: {e}"))?;

    if let Some(identifier) = player {
        let player_id = find_player_by_identifier(&service, &identifier)?;
        let player = service
            .player(&player_id)
            .await
            .map_err(|e| format!("Failed to get player '{player_id}': {e}"))?;
        let player_name = player.identity.get();

        service
            .set_active_player(Some(player_id.clone()))
            .await
            .map_err(|e| format!("Failed to set active player: {e}"))?;

        RuntimeState::set_active_player(Some(player_id.bus_name().to_string()))
            .await
            .map_err(|e| format!("Failed to save active player: {e}"))?;

        println!("Set active player to: {player_name}");
        return Ok(());
    }

    if let Some(player) = service.active_player() {
        let name = player.identity.get();
        let status = match player.playback_state.get() {
            PlaybackState::Playing => "Playing",
            PlaybackState::Paused => "Paused",
            PlaybackState::Stopped => "Stopped",
        };

        let track = player.metadata.title.get();

        println!("Active player: {name} - {track} [{status}]");

        Ok(())
    } else {
        println!("No active player set");

        Ok(())
    }
}
