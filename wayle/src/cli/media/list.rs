use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Execute the command
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    let players = proxy
        .list_players()
        .await
        .map_err(|e| format_error("list players", e))?;

    if players.is_empty() {
        println!("No media players found");
        return Ok(());
    }

    println!("Available media players:");

    for (index, (id, identity, state)) in players.iter().enumerate() {
        let status = match state.as_str() {
            "Playing" => "Playing",
            "Paused" => "Paused",
            _ => "Stopped",
        };

        println!("  {}. {} ({}) [{}]", index + 1, identity, id, status);
    }

    Ok(())
}
