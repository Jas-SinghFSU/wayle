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

    let mut info = vec![
        format!("Player: {}", player.identity.get()),
        format!("Status: {:?}", player.playback_state.get()),
    ];

    info.push(format!("Title: {}", player.metadata.title.get()));
    info.push(format!("Artist: {}", player.metadata.artist.get()));
    info.push(format!("Album: {}", player.metadata.album.get()));

    if let Ok(position) = player.position().await {
        let pos_mins = position.as_secs() / 60;
        let pos_secs = position.as_secs() % 60;

        if let Some(length) = player.metadata.length.get() {
            let len_mins = length.as_secs() / 60;
            let len_secs = length.as_secs() % 60;
            info.push(format!(
                "Position: {pos_mins:02}:{pos_secs:02} / {len_mins:02}:{len_secs:02}"
            ));
        } else {
            info.push(format!("Position: {pos_mins:02}:{pos_secs:02}"));
        }
    }

    info.push(format!(
        "Volume: {:.0}%",
        player.volume.get().as_percentage()
    ));
    info.push(format!("Shuffle: {:?}", player.shuffle_mode.get()));
    info.push(format!("Loop: {:?}", player.loop_mode.get()));

    if player.can_play.get() {
        let mut capabilities = vec![];
        if player.can_seek.get() {
            capabilities.push("Seek");
        }
        if player.can_go_next.get() {
            capabilities.push("Next");
        }
        if player.can_go_previous.get() {
            capabilities.push("Previous");
        }
        if !capabilities.is_empty() {
            info.push(format!("Capabilities: {}", capabilities.join(", ")));
        }
    }

    println!("{}", info.join("\n"));

    Ok(())
}
