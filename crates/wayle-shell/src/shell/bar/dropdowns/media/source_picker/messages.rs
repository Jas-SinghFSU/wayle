use std::sync::Arc;

use wayle_media::{MediaService, core::player::Player, types::PlayerId};

pub(crate) struct SourcePickerInit {
    pub media: Arc<MediaService>,
}

#[derive(Debug)]
pub(crate) enum SourcePickerInput {
    BackClicked,
    SourceSelected(usize),
}

#[derive(Debug)]
pub(crate) enum SourcePickerOutput {
    NavigateBack,
}

#[derive(Debug)]
pub(crate) enum SourcePickerCmd {
    PlayerListChanged {
        players: Vec<Arc<Player>>,
        active_id: Option<PlayerId>,
    },
}
