use wayle_media::MediaProxy;

use super::proxy::format_error;

/// Resolves a player identifier to a full D-Bus bus name.
///
/// Accepts either:
/// - A 1-based index (e.g., "1", "2")
/// - A partial name match (e.g., "spotify", "spot")
/// - An empty string (returns empty, meaning "active player")
///
/// # Errors
/// Returns error if no players are available or no match is found.
pub async fn resolve_player(
    proxy: &MediaProxy<'_>,
    input: Option<String>,
) -> Result<String, String> {
    let input = match input {
        Some(s) if !s.is_empty() => s,
        _ => return Ok(String::new()),
    };

    let players = proxy
        .list_players()
        .await
        .map_err(|e| format_error("list players", e))?;

    if players.is_empty() {
        return Err("No media players available".to_string());
    }

    if let Ok(index) = input.parse::<usize>() {
        if index == 0 {
            return Err("Player numbers start at 1".to_string());
        }
        if let Some((id, _, _)) = players.get(index - 1) {
            return Ok(id.clone());
        }
        return Err(format!(
            "Player {index} not found (available: 1-{})",
            players.len()
        ));
    }

    let input_lower = input.to_lowercase();

    for (id, identity, _) in &players {
        if id.to_lowercase().contains(&input_lower)
            || identity.to_lowercase().contains(&input_lower)
        {
            return Ok(id.clone());
        }
    }

    let available: Vec<_> = players
        .iter()
        .enumerate()
        .map(|(i, (_, identity, _))| format!("{}. {}", i + 1, identity))
        .collect();

    Err(format!(
        "No player matching '{}'\nAvailable players:\n  {}",
        input,
        available.join("\n  ")
    ))
}
