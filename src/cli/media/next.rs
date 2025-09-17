use super::utils::get_player_or_active;
use crate::{cli::CliAction, services::media::MediaService};

/// Execute the command
///
/// # Errors
/// Returns error if service communication fails or player is not found.
pub async fn execute(player: Option<String>) -> CliAction {
    let service = MediaService::new()
        .await
        .map_err(|e| format!("Failed to start media service: {e}"))?;

    let player = get_player_or_active(&service, player.as_ref()).await?;

    player
        .next()
        .await
        .map_err(|e| format!("Failed to skip to next track: {e}"))?;

    Ok(())
}
