use std::{sync::Arc, time::Duration};

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_audio::AudioService;
use wayle_common::{watch, watch_cancellable_throttled};

use crate::shell::bar::dropdowns::audio::main_section::default_devices::volume_section::{
    VolumeSection,
    messages::{ActiveDevice, VolumeSectionCmd, VolumeSectionKind},
};

const VOLUME_THROTTLE: Duration = Duration::from_millis(30);

pub(super) fn spawn_default_device(
    sender: &ComponentSender<VolumeSection>,
    audio: &Arc<AudioService>,
    kind: VolumeSectionKind,
) {
    match kind {
        VolumeSectionKind::Output => {
            let default_output = audio.default_output.clone();
            watch!(sender, [default_output.watch()], |out| {
                let _ = out.send(VolumeSectionCmd::DeviceChanged(
                    default_output.get().map(ActiveDevice::Output),
                ));
            });

            let output_devices = audio.output_devices.clone();
            watch!(sender, [output_devices.watch()], |out| {
                let _ = out.send(VolumeSectionCmd::DeviceChanged(None));
            });
        }
        VolumeSectionKind::Input => {
            let default_input = audio.default_input.clone();
            watch!(sender, [default_input.watch()], |out| {
                let _ = out.send(VolumeSectionCmd::DeviceChanged(
                    default_input
                        .get()
                        .filter(|device| !device.is_monitor.get())
                        .map(ActiveDevice::Input),
                ));
            });

            let input_devices = audio.input_devices.clone();
            watch!(sender, [input_devices.watch()], |out| {
                let _ = out.send(VolumeSectionCmd::DeviceChanged(None));
            });
        }
    }
}

pub(super) fn spawn_device(
    sender: &ComponentSender<VolumeSection>,
    device: &ActiveDevice,
    token: CancellationToken,
) {
    match device {
        ActiveDevice::Output(d) => {
            let volume = d.volume.clone();
            let muted = d.muted.clone();
            watch_cancellable_throttled!(
                sender,
                token,
                VOLUME_THROTTLE,
                [volume.watch(), muted.watch()],
                |out| {
                    let _ = out.send(VolumeSectionCmd::VolumeOrMuteChanged);
                }
            );
        }
        ActiveDevice::Input(d) => {
            let volume = d.volume.clone();
            let muted = d.muted.clone();
            watch_cancellable_throttled!(
                sender,
                token,
                VOLUME_THROTTLE,
                [volume.watch(), muted.watch()],
                |out| {
                    let _ = out.send(VolumeSectionCmd::VolumeOrMuteChanged);
                }
            );
        }
    }
}
