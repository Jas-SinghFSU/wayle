use std::sync::Arc;

use relm4::ComponentSender;
use wayle_media::{core::player::Player, types::PlayerId};

use super::{SourcePicker, SourcePickerCmd, source_item::SourceItemInit};
use crate::shell::bar::dropdowns::media::helpers;

impl SourcePicker {
    pub(super) fn select_source(&self, index: usize, sender: &ComponentSender<SourcePicker>) {
        let Some(player_id) = self
            .sources
            .get(index)
            .map(|source_item| source_item.player_id())
        else {
            return;
        };

        let media = self.media.clone();
        sender.oneshot_command(async move {
            if let Err(err) = media.set_active_player(Some(player_id)).await {
                tracing::warn!(error = %err, "set active player failed");
            }
            SourcePickerCmd::PlayerListChanged {
                players: media.player_list.get(),
                active_id: media.active_player.get().map(|player| player.id.clone()),
            }
        });
    }

    pub(super) fn rebuild_source_list(
        &mut self,
        players: &[Arc<Player>],
        active_id: Option<&PlayerId>,
    ) {
        let mut guard = self.sources.guard();
        guard.clear();

        for player in players {
            let is_active = active_id.is_some_and(|active| *active == player.id);

            guard.push_back(SourceItemInit {
                identity: player.identity.get(),
                player_id: player.id.clone(),
                icon_name: helpers::resolve_source_icon(player),
                active: is_active,
            });
        }
    }
}
