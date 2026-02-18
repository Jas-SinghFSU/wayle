use std::sync::Arc;

use wayle_audio::AudioService;

use super::volume_section::VolumeSectionOutput;

pub(crate) struct DefaultDevicesInit {
    pub audio: Arc<AudioService>,
}

#[derive(Debug)]
pub(crate) enum DefaultDevicesInput {
    OutputSection(VolumeSectionOutput),
    InputSection(VolumeSectionOutput),
}

#[derive(Debug)]
pub(crate) enum DefaultDevicesOutput {
    ShowOutputDevices,
    ShowInputDevices,
    HasDeviceChanged(bool),
}
