use std::sync::Arc;

use futures::StreamExt;
use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_common::{watch, watch_cancellable};
use wayle_media::{MediaService, core::player::Player};

use super::{PlayerView, PlayerViewCmd};

pub(super) fn spawn_static(sender: &ComponentSender<PlayerView>, media: &Arc<MediaService>) {
    let active_player = media.active_player.clone();
    watch!(sender, [active_player.watch()], |out| {
        let _ = out.send(PlayerViewCmd::PlayerChanged(active_player.get()));
    });
}

pub(super) fn spawn_player(
    sender: &ComponentSender<PlayerView>,
    player: &Arc<Player>,
    token: CancellationToken,
) {
    let metadata = player.metadata.clone();
    let metadata_token = token.clone();
    watch_cancellable!(sender, metadata_token, [metadata.watch()], |out| {
        let _ = out.send(PlayerViewCmd::MetadataChanged);
    });

    let cover_art = player.metadata.cover_art.clone();
    let cover_art_token = token.clone();
    watch_cancellable!(sender, cover_art_token, [cover_art.watch()], |out| {
        let _ = out.send(PlayerViewCmd::CoverArtChanged(cover_art.get()));
    });

    let playback_state = player.playback_state.clone();
    let state_token = token.clone();
    watch_cancellable!(sender, state_token, [playback_state.watch()], |out| {
        let _ = out.send(PlayerViewCmd::PlaybackStateChanged(playback_state.get()));
    });

    let loop_mode = player.loop_mode.clone();
    let loop_token = token.clone();
    watch_cancellable!(sender, loop_token, [loop_mode.watch()], |out| {
        let _ = out.send(PlayerViewCmd::LoopModeChanged(loop_mode.get()));
    });

    let shuffle_mode = player.shuffle_mode.clone();
    let shuffle_token = token.clone();
    watch_cancellable!(sender, shuffle_token, [shuffle_mode.watch()], |out| {
        let _ = out.send(PlayerViewCmd::ShuffleModeChanged(shuffle_mode.get()));
    });

    let can_go_next = player.can_go_next.clone();
    let can_go_previous = player.can_go_previous.clone();
    let can_seek = player.can_seek.clone();
    let can_loop = player.can_loop.clone();
    let can_shuffle = player.can_shuffle.clone();
    let caps_token = token.clone();
    watch_cancellable!(
        sender,
        caps_token,
        [
            can_go_next.watch(),
            can_go_previous.watch(),
            can_seek.watch(),
            can_loop.watch(),
            can_shuffle.watch(),
        ],
        |out| {
            let _ = out.send(PlayerViewCmd::CapabilitiesChanged);
        }
    );

    let position_player = player.clone();
    sender.command(move |out, shutdown| async move {
        let shutdown_fut = shutdown.wait();
        tokio::pin!(shutdown_fut);
        let mut stream = Box::pin(position_player.watch_position());
        loop {
            tokio::select! {
                () = &mut shutdown_fut => break,
                _ = token.cancelled() => break,
                Some(position) = stream.next() => {
                    let _ = out.send(PlayerViewCmd::PositionTick(position));
                }
                else => break,
            }
        }
    });
}
