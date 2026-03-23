use std::sync::Arc;

use futures::StreamExt;
use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_media::{MediaService, core::player::Player};
use wayle_widgets::{watch, watch_cancellable};

use super::{MediaSection, messages::MediaSectionCmd};

pub(super) fn spawn(sender: &ComponentSender<MediaSection>, media: &Option<Arc<MediaService>>) {
    let Some(media) = media else {
        return;
    };

    let active_player = media.active_player.clone();

    watch!(sender, [active_player.watch()], |out| {
        let _ = out.send(MediaSectionCmd::PlayerChanged);
    });

    let player_list = media.player_list.clone();

    watch!(sender, [player_list.watch()], |out| {
        let _ = out.send(MediaSectionCmd::PlayerListChanged(player_list.get().len()));
    });
}

pub(super) fn spawn_player_watchers(
    sender: &ComponentSender<MediaSection>,
    player: &Arc<Player>,
    token: CancellationToken,
) {
    let metadata = player.metadata.clone();

    watch_cancellable!(sender, token.clone(), [metadata.watch()], |out| {
        let _ = out.send(MediaSectionCmd::MetadataChanged {
            title: metadata.title.get(),
            artist: metadata.artist.get(),
            cover_art: metadata.cover_art.get(),
            length: metadata.length.get(),
        });
    });

    let playback_state = player.playback_state.clone();

    watch_cancellable!(sender, token.clone(), [playback_state.watch()], |out| {
        let _ = out.send(MediaSectionCmd::PlaybackStateChanged(playback_state.get()));
    });

    let can_seek = player.can_seek.clone();

    watch_cancellable!(sender, token.clone(), [can_seek.watch()], |out| {
        let _ = out.send(MediaSectionCmd::CanSeekChanged(can_seek.get()));
    });

    let position_player = player.clone();

    sender.command(move |out, shutdown| async move {
        let shutdown_fut = shutdown.wait();
        tokio::pin!(shutdown_fut);

        let mut stream = Box::pin(position_player.position.watch());

        loop {
            tokio::select! {
                () = &mut shutdown_fut => break,
                _ = token.cancelled() => break,
                Some(position) = stream.next() => {
                    let _ = out.send(MediaSectionCmd::PositionTick(position));
                }
                else => break,
            }
        }
    });
}
