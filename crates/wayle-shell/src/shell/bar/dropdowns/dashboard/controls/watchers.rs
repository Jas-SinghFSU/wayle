use std::sync::Arc;

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_audio::AudioService;
use wayle_widgets::{watch, watch_cancellable};

use super::{ControlsSection, messages::ControlsCmd};

pub(super) fn spawn(sender: &ComponentSender<ControlsSection>, audio: &Option<Arc<AudioService>>) {
    let Some(audio) = audio else {
        return;
    };

    let default_output = audio.default_output.clone();

    watch!(sender, [default_output.watch()], |out| {
        match default_output.get() {
            Some(device) => {
                let _ = out.send(ControlsCmd::DeviceAvailable(true));
                let _ = out.send(ControlsCmd::DeviceNameChanged(device.description.get()));
                let _ = out.send(ControlsCmd::VolumeChanged(
                    device.volume.get().average_percentage(),
                ));
                let _ = out.send(ControlsCmd::MuteChanged(device.muted.get()));
            }
            None => {
                let _ = out.send(ControlsCmd::DeviceAvailable(false));
            }
        }
    });
}

pub(super) fn spawn_device_watchers(
    sender: &ComponentSender<ControlsSection>,
    audio: &Arc<AudioService>,
    token: CancellationToken,
) {
    let Some(device) = audio.default_output.get() else {
        return;
    };

    let volume = device.volume.clone();

    watch_cancellable!(sender, token.clone(), [volume.watch()], |out| {
        let _ = out.send(ControlsCmd::VolumeChanged(
            volume.get().average_percentage(),
        ));
    });

    let muted = device.muted.clone();

    watch_cancellable!(sender, token, [muted.watch()], |out| {
        let _ = out.send(ControlsCmd::MuteChanged(muted.get()));
    });
}
