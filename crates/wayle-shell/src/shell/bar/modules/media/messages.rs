use std::sync::Arc;

use wayle_media::{core::player::Player, types::PlaybackState};
use wayle_widgets::prelude::BarSettings;

pub struct MediaInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum MediaMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub enum MediaCmd {
    PlayerChanged(Option<Arc<Player>>),
    MetadataChanged,
    PlaybackStateChanged(PlaybackState),
    UpdateIcon(String),
    IconTypeChanged,
}
