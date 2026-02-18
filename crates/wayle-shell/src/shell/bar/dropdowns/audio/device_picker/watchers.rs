use std::sync::Arc;

use relm4::ComponentSender;
use wayle_audio::AudioService;
use wayle_common::watch;

use super::{
    DevicePicker,
    messages::DevicePickerCmd,
    methods::{build_input_device_list, build_output_device_list},
};
use crate::shell::bar::dropdowns::audio::VolumeSectionKind;

pub(super) fn spawn(
    sender: &ComponentSender<DevicePicker>,
    audio: &Arc<AudioService>,
    kind: VolumeSectionKind,
) {
    match kind {
        VolumeSectionKind::Output => {
            let output_devices = audio.output_devices.clone();
            let default_output = audio.default_output.clone();
            watch!(
                sender,
                [output_devices.watch(), default_output.watch()],
                |out| {
                    let list =
                        build_output_device_list(&output_devices.get(), &default_output.get());
                    let _ = out.send(DevicePickerCmd::DevicesChanged(list));
                }
            );
        }
        VolumeSectionKind::Input => {
            let input_devices = audio.input_devices.clone();
            let default_input = audio.default_input.clone();
            watch!(
                sender,
                [input_devices.watch(), default_input.watch()],
                |out| {
                    let list = build_input_device_list(&input_devices.get(), &default_input.get());
                    let _ = out.send(DevicePickerCmd::DevicesChanged(list));
                }
            );
        }
    }
}
