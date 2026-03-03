use std::{sync::Arc, time::Duration};

use wayle_media::{
    MediaService,
    core::player::Player,
    types::{LoopMode, PlaybackState, ShuffleMode},
};

pub(crate) struct PlayerViewInit {
    pub media: Arc<MediaService>,
}

#[derive(Debug)]
pub(crate) enum PlayerViewInput {
    ShowSourcePickerClicked,
    PlayPauseClicked,
    NextClicked,
    PreviousClicked,
    ShuffleClicked,
    LoopClicked,
    SeekCommitted(f64),
}

#[derive(Debug)]
pub(crate) enum PlayerViewOutput {
    ShowSourcePicker,
}

#[derive(Debug)]
pub(crate) enum PlayerViewCmd {
    PlayerChanged(Option<Arc<Player>>),
    MetadataChanged,
    CoverArtChanged(Option<String>),
    PlaybackStateChanged(PlaybackState),
    PositionTick(Duration),
    CapabilitiesChanged,
    LoopModeChanged(LoopMode),
    ShuffleModeChanged(ShuffleMode),
    Noop,
}
