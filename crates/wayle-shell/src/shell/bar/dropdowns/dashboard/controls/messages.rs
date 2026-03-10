use std::sync::Arc;

use wayle_audio::AudioService;

pub(crate) struct ControlsInit {
    pub audio: Option<Arc<AudioService>>,
}

#[derive(Debug)]
pub(crate) enum ControlsInput {
    VolumeCommitted(f64),
    MuteToggled,
}

#[derive(Debug)]
pub(crate) enum ControlsCmd {
    VolumeChanged(f64),
    MuteChanged(bool),
    DeviceNameChanged(String),
    DeviceAvailable(bool),
}
