use std::sync::Arc;

use relm4::ComponentSender;
use wayle_media::MediaService;
use wayle_widgets::watch;

use super::{SourcePicker, SourcePickerCmd};

pub(super) fn spawn(sender: &ComponentSender<SourcePicker>, media: &Arc<MediaService>) {
    let player_list = media.player_list.clone();
    let active_player = media.active_player.clone();
    watch!(
        sender,
        [player_list.watch(), active_player.watch()],
        |out| {
            let _ = out.send(SourcePickerCmd::PlayerListChanged {
                players: player_list.get(),
                active_id: active_player.get().map(|player| player.id.clone()),
            });
        }
    );
}
