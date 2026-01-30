use std::sync::Arc;

use wayle_media::core::player::Player;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct MediaInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub(crate) enum MediaMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum MediaCmd {
    PlayerChanged(Option<Arc<Player>>),
    MetadataChanged,
    PlaybackStateChanged,
    UpdateIcon(String),
    IconTypeChanged,
}
