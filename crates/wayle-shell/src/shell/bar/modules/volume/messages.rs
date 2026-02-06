use std::sync::Arc;

use wayle_audio::{AudioService, core::device::output::OutputDevice};
use wayle_config::ConfigService;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct VolumeInit {
    pub settings: BarSettings,
    pub audio: Arc<AudioService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum VolumeMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum VolumeCmd {
    DeviceChanged(Option<Arc<OutputDevice>>),
    VolumeOrMuteChanged,
    IconConfigChanged,
}
