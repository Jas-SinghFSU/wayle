use super::{
    proxy::{connect, format_error},
    resolve::resolve_player,
};
use crate::cli::CliAction;

/// Execute the command
///
/// # Errors
/// Returns error if D-Bus communication fails or player is not found.
pub async fn execute(player: Option<String>) -> CliAction {
    let (_connection, proxy) = connect().await?;

    let resolved = resolve_player(&proxy, player).await?;

    let info = proxy
        .get_player_info(resolved)
        .await
        .map_err(|e| format_error("get player info", e))?;

    let mut output = vec![
        format!(
            "Player: {}",
            info.get("identity")
                .map(String::as_str)
                .unwrap_or("Unknown")
        ),
        format!(
            "Status: {}",
            info.get("playback_state")
                .map(String::as_str)
                .unwrap_or("Unknown")
        ),
    ];

    output.push(format!(
        "Title: {}",
        info.get("title").map(String::as_str).unwrap_or("Unknown")
    ));
    output.push(format!(
        "Artist: {}",
        info.get("artist").map(String::as_str).unwrap_or("Unknown")
    ));
    output.push(format!(
        "Album: {}",
        info.get("album").map(String::as_str).unwrap_or("Unknown")
    ));

    if let Some(length_us) = info.get("length_us")
        && let Ok(us) = length_us.parse::<u64>() {
            let secs = us / 1_000_000;
            let len_mins = secs / 60;
            let len_secs = secs % 60;
            output.push(format!("Length: {len_mins:02}:{len_secs:02}"));
        }

    output.push(format!(
        "Volume: {}%",
        info.get("volume").map(String::as_str).unwrap_or("0")
    ));
    output.push(format!(
        "Shuffle: {}",
        info.get("shuffle_mode")
            .map(String::as_str)
            .unwrap_or("Unknown")
    ));
    output.push(format!(
        "Loop: {}",
        info.get("loop_mode")
            .map(String::as_str)
            .unwrap_or("Unknown")
    ));

    let mut capabilities = vec![];
    if info.get("can_seek").map(|s| s == "true").unwrap_or(false) {
        capabilities.push("Seek");
    }
    if info
        .get("can_go_next")
        .map(|s| s == "true")
        .unwrap_or(false)
    {
        capabilities.push("Next");
    }
    if info
        .get("can_go_previous")
        .map(|s| s == "true")
        .unwrap_or(false)
    {
        capabilities.push("Previous");
    }
    if !capabilities.is_empty() {
        output.push(format!("Capabilities: {}", capabilities.join(", ")));
    }

    println!("{}", output.join("\n"));

    Ok(())
}
