use crate::{
    cli::CliAction,
    services::media::{
        service::{Config, MediaService},
        types::PlaybackState,
    },
};

/// Execute the command
pub async fn execute() -> CliAction {
    let service = MediaService::new(Config {
        ignored_players: vec![],
    })
    .await
    .map_err(|e| format!("Failed to start media service: {e}"))?;

    let players = service.players();

    if players.is_empty() {
        println!("No media players found");
        return Ok(());
    }

    println!("Available media players:");

    for (index, player) in players.iter().enumerate() {
        let status = match player.playback_state.get() {
            PlaybackState::Playing => "▶ Playing",
            PlaybackState::Paused => "⏸ Paused",
            PlaybackState::Stopped => "⏹ Stopped",
        };

        println!(
            "  {}. {} - {} [{}]",
            index + 1,
            player.identity.get(),
            player.metadata.title.get(),
            status
        );
    }

    Ok(())
}
