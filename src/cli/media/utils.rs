use std::sync::Arc;

use crate::services::media::{core::player::Player, service::MediaService, types::PlayerId};

/// Finds a player by identifier (index or partial name match)
///
/// # Errors
/// Returns error if no players exist, invalid index provided, or multiple matches found.
pub fn find_player_by_identifier(
    service: &MediaService,
    identifier: &str,
) -> Result<PlayerId, String> {
    let players = service.players();

    if players.is_empty() {
        return Err(String::from("No media players found"));
    }

    if let Ok(index) = identifier.parse::<usize>() {
        if index > 0 && index <= players.len() {
            return Ok(players[index - 1].id.clone());
        } else {
            return Err(format!(
                "Invalid player index. Valid range: 1-{}",
                players.len()
            ));
        }
    }

    let identifier_lower = identifier.to_lowercase();
    let mut matches = Vec::new();

    for player in &players {
        let identity_lower = player.identity.get().to_lowercase();
        let bus_name_lower = player.id.bus_name().to_lowercase();

        if identity_lower.contains(&identifier_lower) || bus_name_lower.contains(&identifier_lower)
        {
            matches.push((player.id.clone(), player.identity.get()));
        }
    }

    match matches.len() {
        0 => Err(format!("No player found matching '{identifier}'")),
        1 => Ok(matches[0].0.clone()),
        _ => {
            let names: Vec<String> = matches.iter().map(|(_, name)| name.clone()).collect();
            Err(format!(
                "Multiple players match '{}': {}. Please be more specific.",
                identifier,
                names.join(", ")
            ))
        }
    }
}

/// Gets a player from optional identifier or returns active player
///
/// # Errors
/// Returns error if player identification fails, setting active player fails, or no active player exists.
pub async fn get_player_or_active(
    service: &MediaService,
    identifier: Option<&String>,
) -> Result<Arc<Player>, String> {
    if let Some(id) = identifier {
        let player_id = find_player_by_identifier(service, id)?;

        service
            .set_active_player(Some(player_id.clone()))
            .await
            .map_err(|e| format!("Failed to set active player: {e}"))?;

        service
            .player_monitored(&player_id)
            .await
            .map_err(|e| format!("Failed to get player '{player_id}': {e}"))
    } else {
        service.active_player().ok_or_else(|| {
            String::from(
                "No active player. Specify a player or set one with 'wayle media active <player>'.",
            )
        })
    }
}
