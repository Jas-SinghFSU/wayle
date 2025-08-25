use super::{commands::ShuffleModeArg, utils::get_player_or_active};
use crate::{
    cli::CliAction,
    services::media::{Config, MediaService, ShuffleMode},
};

pub async fn execute(state: Option<ShuffleModeArg>, player: Option<String>) -> CliAction {
    let service = MediaService::start(Config {
        ignored_players: vec![],
    })
    .await
    .map_err(|e| format!("Failed to start media service: {e}"))?;

    let player = get_player_or_active(&service, player.as_ref()).await?;

    let new_mode = match state {
        Some(ShuffleModeArg::On) => ShuffleMode::On,
        Some(ShuffleModeArg::Off) => ShuffleMode::Off,
        Some(ShuffleModeArg::Toggle) | None => match player.shuffle_mode.get() {
            ShuffleMode::On => ShuffleMode::Off,
            ShuffleMode::Off | ShuffleMode::Unsupported => ShuffleMode::On,
        },
    };

    player
        .set_shuffle_mode(new_mode)
        .await
        .map_err(|e| format!("Failed to set shuffle: {e}"))?;

    Ok(())
}
