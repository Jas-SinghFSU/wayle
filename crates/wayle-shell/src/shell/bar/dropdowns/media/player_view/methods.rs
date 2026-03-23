use std::{borrow::Cow, future::Future, sync::Arc, time::Duration};

use relm4::ComponentSender;
use wayle_media::{core::player::Player, types::*};

use super::{PlayerView, PlayerViewCmd};
use crate::{i18n::t, shell::bar::dropdowns::media::helpers};

impl PlayerView {
    pub(super) fn set_active(&mut self, active: bool, sender: &ComponentSender<PlayerView>) {
        if self.is_active == active {
            return;
        }

        self.is_active = active;
        let _ = self.player_watcher.reset();

        if active {
            self.update_player(self.player.clone(), sender);
        }
    }

    pub(super) fn fire_player_command<F, Fut>(
        &self,
        sender: &ComponentSender<PlayerView>,
        action: F,
        label: &'static str,
    ) where
        F: FnOnce(Arc<Player>) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), wayle_media::Error>> + Send,
    {
        let Some(player) = self.player.clone() else {
            return;
        };
        sender.oneshot_command(async move {
            if let Err(err) = action(player).await {
                tracing::warn!(error = %err, "{label}");
            }
            PlayerViewCmd::Noop
        });
    }

    pub(super) fn update_player(
        &mut self,
        player: Option<Arc<Player>>,
        sender: &ComponentSender<PlayerView>,
    ) {
        self.player = player.clone();
        self.has_player = player.is_some();

        let Some(player) = player else {
            self.clear_fields();
            return;
        };

        self.refresh_from_player(&player);

        let token = self.player_watcher.reset();
        if self.is_active {
            super::watchers::spawn_player(sender, &player, token);
            self.refresh_position_now(sender);
        }
    }

    pub(super) fn refresh_metadata(&mut self) {
        let Some(player) = self.player.as_ref() else {
            return;
        };

        let metadata = &player.metadata;
        self.title = metadata.title.get();
        self.artist = metadata.artist.get();
        self.album = metadata.album.get();
        self.length = metadata.length.get();
        self.cover_art = metadata.cover_art.get();
        self.update_artwork_css();
    }

    pub(super) fn refresh_capabilities(&mut self) {
        let Some(player) = self.player.as_ref() else {
            return;
        };

        self.can_go_previous = player.can_go_previous.get();
        self.can_go_next = player.can_go_next.get();
        self.can_seek = player.can_seek.get();
        self.can_loop = player.can_loop.get();
        self.can_shuffle = player.can_shuffle.get();
    }

    pub(super) fn display_title(&self) -> Cow<'_, str> {
        if self.title.is_empty() {
            Cow::Owned(t!("dropdown-media-unknown-title"))
        } else {
            Cow::Borrowed(&self.title)
        }
    }

    pub(super) fn display_artist(&self) -> Cow<'_, str> {
        if self.artist.is_empty() {
            Cow::Owned(t!("dropdown-media-unknown-artist"))
        } else {
            Cow::Borrowed(&self.artist)
        }
    }

    pub(super) fn display_album(&self) -> Cow<'_, str> {
        if self.album.is_empty() {
            Cow::Owned(t!("dropdown-media-unknown-album"))
        } else {
            Cow::Borrowed(&self.album)
        }
    }

    pub(super) fn progress_fraction(&self) -> f64 {
        let Some(length) = self.length else {
            return 0.0;
        };
        helpers::progress_fraction(self.position, length)
    }

    pub(super) fn update_artwork_css(&self) {
        let css = match self.cover_art.as_deref() {
            Some(path) => {
                format!(
                    ".{} {{ background-image: url(\"file://{path}\"); }}",
                    self.art_css_class
                )
            }
            None => format!(".{} {{ background-image: none; }}", self.art_css_class),
        };
        self.art_css_provider.load_from_string(&css);
    }

    pub(super) fn play_pause_icon(&self) -> &'static str {
        match self.playback_state {
            PlaybackState::Playing => "ld-pause-symbolic",
            PlaybackState::Paused | PlaybackState::Stopped => "ld-play-symbolic",
        }
    }

    pub(super) fn loop_icon(&self) -> &'static str {
        match self.loop_mode {
            LoopMode::Track => "ld-repeat-1-symbolic",
            LoopMode::None | LoopMode::Playlist | LoopMode::Unsupported => "ld-repeat-symbolic",
        }
    }

    fn refresh_from_player(&mut self, player: &Player) {
        self.player_identity = player.identity.get();
        self.playback_state = player.playback_state.get();
        self.loop_mode = player.loop_mode.get();
        self.shuffle_mode = player.shuffle_mode.get();

        self.can_go_previous = player.can_go_previous.get();
        self.can_go_next = player.can_go_next.get();
        self.can_seek = player.can_seek.get();
        self.can_loop = player.can_loop.get();
        self.can_shuffle = player.can_shuffle.get();

        self.source_icon = helpers::resolve_source_icon(player);
        self.refresh_metadata();
        self.position = player.position.get();
        self.seek_slider.set_value(self.progress_fraction() * 100.0);
    }

    fn refresh_position_now(&self, sender: &ComponentSender<PlayerView>) {
        let Some(player) = self.player.clone() else {
            return;
        };

        sender.oneshot_command(async move {
            match player.position().await {
                Ok(position) => PlayerViewCmd::PositionTick(position),
                Err(error) => {
                    tracing::debug!(error = %error, "immediate position refresh failed");
                    PlayerViewCmd::Noop
                }
            }
        });
    }

    fn clear_fields(&mut self) {
        self.title.clear();
        self.artist.clear();
        self.album.clear();
        self.cover_art = None;
        self.playback_state = PlaybackState::Stopped;
        self.position = Duration::ZERO;
        self.length = None;
        self.loop_mode = LoopMode::None;
        self.shuffle_mode = ShuffleMode::Off;
        self.can_go_previous = false;
        self.can_go_next = false;
        self.can_seek = false;
        self.can_loop = false;
        self.can_shuffle = false;
        self.player_identity.clear();
        self.source_icon = String::from("ld-music-symbolic");
        self.seek_slider.set_value(0.0);

        self.update_artwork_css();
    }
}
