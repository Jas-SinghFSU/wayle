use std::sync::Arc;

use wayle_audio::core::device::output::OutputDevice;
use wayle_widgets::prelude::BarSettings;

pub struct VolumeInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum VolumeMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum VolumeCmd {
    DeviceChanged(Option<Arc<OutputDevice>>),
    VolumeOrMuteChanged,
    IconConfigChanged,
}
