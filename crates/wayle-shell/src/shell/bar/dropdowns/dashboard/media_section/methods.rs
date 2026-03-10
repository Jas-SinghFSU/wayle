use std::{future::Future, sync::Arc, time::Duration};

use relm4::ComponentSender;
use tracing::{debug, warn};
use wayle_media::core::player::Player;

use super::{MediaSection, PERCENTAGE_SCALE, messages::MediaSectionCmd, watchers};

impl MediaSection {
    pub(super) fn progress_fraction(&self) -> f64 {
        match self.length {
            Some(length) if !length.is_zero() => self.position.as_secs_f64() / length.as_secs_f64(),
            _ => 0.0,
        }
    }

    pub(super) fn update_artwork_css(&self) {
        let css = match self.cover_art.as_deref() {
            Some(path) => format!(
                ".{} {{ background-image: url(\"file://{path}\"); }}",
                self.art_css_class
            ),
            None => format!(".{} {{ background-image: none; }}", self.art_css_class),
        };
        self.art_css_provider.load_from_string(&css);
    }

    pub(super) fn handle_player_changed(&mut self, sender: &ComponentSender<Self>) {
        let active = self
            .media
            .as_ref()
            .and_then(|media| media.active_player.get());

        let Some(player) = active else {
            self.has_player = false;
            self.player = None;
            self.cover_art = None;
            self.position = Duration::ZERO;
            self.length = None;
            self.can_seek = false;
            self.update_artwork_css();
            self.seek_slider.set_value(0.0);
            let _ = self.player_watcher.reset();
            return;
        };

        self.title = player.metadata.title.get();
        self.artist = player.metadata.artist.get();
        self.cover_art = player.metadata.cover_art.get();
        self.length = player.metadata.length.get();
        self.position = player.position.get();

        self.playback_state = player.playback_state.get();
        self.can_previous = player.can_go_previous.get();
        self.can_next = player.can_go_next.get();
        self.can_seek = player.can_seek.get();
        self.has_player = true;

        self.update_artwork_css();
        self.seek_slider
            .set_value(self.progress_fraction() * PERCENTAGE_SCALE);

        let token = self.player_watcher.reset();

        if self.is_active {
            watchers::spawn_player_watchers(sender, &player, token);
        }

        self.player = Some(player);

        if self.is_active {
            self.refresh_position_now(sender);
        }
    }

    pub(super) fn fire_player_command<F, Fut>(
        &self,
        sender: &ComponentSender<Self>,
        command: F,
        error_context: &'static str,
    ) where
        F: FnOnce(Arc<Player>) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), wayle_media::Error>> + Send,
    {
        let Some(player) = self.player.clone() else {
            return;
        };

        sender.oneshot_command(async move {
            if let Err(err) = command(player).await {
                warn!(error = %err, error_context);
            }
            MediaSectionCmd::PlayerChanged
        });
    }

    pub(super) fn cycle_player(&self, sender: &ComponentSender<Self>) {
        let Some(media) = self.media.clone() else {
            return;
        };
        let Some(current_player) = self.player.clone() else {
            return;
        };

        sender.oneshot_command(async move {
            let players = media.player_list.get();

            if players.len() < 2 {
                return MediaSectionCmd::PlayerChanged;
            }

            let current_index = players
                .iter()
                .position(|player| player.id == current_player.id)
                .unwrap_or(0);

            let next_index = (current_index + 1) % players.len();
            let next_player_id = players[next_index].id.clone();

            if let Err(err) = media.set_active_player(Some(next_player_id)).await {
                warn!(error = %err, "switch player failed");
            }

            MediaSectionCmd::PlayerChanged
        });
    }

    pub(super) fn refresh_position_now(&self, sender: &ComponentSender<Self>) {
        let Some(player) = self.player.clone() else {
            return;
        };

        sender.oneshot_command(async move {
            match player.position().await {
                Ok(position) => MediaSectionCmd::PositionTick(position),
                Err(error) => {
                    debug!(error = %error, "immediate dashboard position refresh failed");
                    MediaSectionCmd::Noop
                }
            }
        });
    }
}
