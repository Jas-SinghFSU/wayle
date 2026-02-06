use std::sync::Arc;

use wayle_audio::{AudioService, core::device::input::InputDevice};
use wayle_config::ConfigService;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct MicrophoneInit {
    pub settings: BarSettings,
    pub audio: Arc<AudioService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum MicrophoneMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum MicrophoneCmd {
    DeviceChanged(Option<Arc<InputDevice>>),
    VolumeOrMuteChanged,
    IconConfigChanged,
}
