use std::sync::Arc;

use wayle_audio::AudioService;
use wayle_config::ConfigService;

use super::{device_picker::DevicePickerOutput, main_section::MainSectionOutput};

pub(crate) struct AudioDropdownInit {
    pub audio: Arc<AudioService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum AudioDropdownMsg {
    MainSection(MainSectionOutput),
    OutputPicker(DevicePickerOutput),
    InputPicker(DevicePickerOutput),
}

#[derive(Debug)]
pub(crate) enum AudioDropdownCmd {
    ScaleChanged(f32),
}
