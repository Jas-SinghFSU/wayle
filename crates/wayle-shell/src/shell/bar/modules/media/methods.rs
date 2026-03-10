use relm4::{gtk, gtk::prelude::*};
use wayle_media::types::PlaybackState;

use super::MediaModule;

impl MediaModule {
    pub(super) fn update_disc_mode(root: &gtk::Box, enabled: bool) {
        if enabled {
            root.add_css_class("media-disc");
        } else {
            root.remove_css_class("media-disc");
        }
    }

    pub(super) fn update_spinning_state(root: &gtk::Box, state: PlaybackState) {
        match state {
            PlaybackState::Playing => {
                root.add_css_class("media-spinning");
            }
            PlaybackState::Paused | PlaybackState::Stopped => {
                root.remove_css_class("media-spinning");
            }
        }
    }
}
