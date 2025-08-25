use std::time::Duration;

use super::utils::get_player_or_active;
use crate::{
    cli::CliAction,
    services::media::{Config, MediaService},
};

pub async fn execute(position: String, player: Option<String>) -> CliAction {
    let service = MediaService::start(Config {
        ignored_players: vec![],
    })
    .await
    .map_err(|e| format!("Failed to start media service: {e}"))?;

    let player = get_player_or_active(&service, player.as_ref()).await?;
    let current_position = player.position().await.ok();
    let track_length = player.metadata.length.get();
    let target_position = parse_position(&position, current_position, track_length)?;

    if let Some(length) = track_length
        && target_position > length
    {
        return Err(format!(
            "Position {target_position:?} exceeds track length {length:?}"
        ));
    }

    player
        .seek(target_position)
        .await
        .map_err(|e| format!("Failed to seek: {e}"))?;

    Ok(())
}

fn parse_position(
    position_str: &str,
    current_position: Option<Duration>,
    track_length: Option<Duration>,
) -> Result<Duration, String> {
    if let Some(percentage_str) = position_str.strip_suffix('%') {
        let percentage = percentage_str
            .parse::<f64>()
            .map_err(|_| "Invalid percentage format".to_string())?;

        if !(0.0..=100.0).contains(&percentage) {
            return Err("Percentage must be between 0 and 100".to_string());
        }

        let track_length = track_length
            .ok_or_else(|| "Cannot use percentage - track length unknown".to_string())?;

        let position_secs = track_length.as_secs_f64() * (percentage / 100.0);
        return Ok(Duration::from_secs_f64(position_secs));
    }

    if position_str.starts_with('+') || position_str.starts_with('-') {
        let current = current_position
            .ok_or_else(|| "Cannot use relative seeking - current position unknown".to_string())?;

        let delta_str = &position_str[1..];
        let delta_secs = delta_str
            .parse::<i64>()
            .map_err(|_| "Invalid relative seek format".to_string())?;

        let new_position = if position_str.starts_with('+') {
            current.saturating_add(Duration::from_secs(delta_secs.unsigned_abs()))
        } else {
            current.saturating_sub(Duration::from_secs(delta_secs.unsigned_abs()))
        };

        return Ok(new_position);
    }

    if position_str.contains(':') {
        let parts: Vec<&str> = position_str.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid time format. Use mm:ss".to_string());
        }

        let minutes = parts[0]
            .parse::<u64>()
            .map_err(|_| "Invalid minutes value".to_string())?;

        let seconds = parts[1]
            .parse::<u64>()
            .map_err(|_| "Invalid seconds value".to_string())?;

        if seconds >= 60 {
            return Err("Seconds must be less than 60".to_string());
        }

        return Ok(Duration::from_secs(minutes * 60 + seconds));
    }

    let seconds = position_str.parse::<u64>().map_err(|_| {
        "Invalid position format. Use seconds, mm:ss, percentage (50%), or relative (+10, -10)"
            .to_string()
    })?;

    Ok(Duration::from_secs(seconds))
}
