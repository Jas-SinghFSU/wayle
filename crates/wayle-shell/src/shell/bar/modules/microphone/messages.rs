use std::sync::Arc;

use wayle_audio::core::device::input::InputDevice;
use wayle_widgets::prelude::BarSettings;

pub struct MicrophoneInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum MicrophoneMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub enum MicrophoneCmd {
    DeviceChanged(Option<Arc<InputDevice>>),
    VolumeOrMuteChanged,
    IconConfigChanged,
}
