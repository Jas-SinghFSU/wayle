use std::sync::Arc;

use wayle_audio::core::device::input::InputDevice;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct MicrophoneInit {
    pub settings: BarSettings,
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
