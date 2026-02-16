use std::sync::Arc;

use wayle_audio::{
    AudioService,
    core::{
        device::{input::InputDevice, output::OutputDevice},
        stream::AudioStream,
    },
};
use wayle_config::ConfigService;

use super::{device_picker::DevicePickerOutput, volume_section::VolumeSectionOutput};

pub(crate) struct AudioDropdownInit {
    pub audio: Arc<AudioService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum AudioDropdownMsg {
    OutputSection(VolumeSectionOutput),
    InputSection(VolumeSectionOutput),
    OutputPicker(DevicePickerOutput),
    InputPicker(DevicePickerOutput),
    AppVolumeChanged(u32, f64),
    ToggleAppMute(u32),
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum AudioDropdownCmd {
    DefaultOutputChanged(Option<Arc<OutputDevice>>),
    DefaultInputChanged(Option<Arc<InputDevice>>),
    OutputVolumeOrMuteChanged,
    InputVolumeOrMuteChanged,
    OutputDevicesChanged(Vec<Arc<OutputDevice>>),
    InputDevicesChanged(Vec<Arc<InputDevice>>),
    PlaybackStreamsChanged(Vec<Arc<AudioStream>>),
    AppStreamPropertyChanged(u32),
    ScaleChanged(f32),
    AppIconSourceChanged,
}
