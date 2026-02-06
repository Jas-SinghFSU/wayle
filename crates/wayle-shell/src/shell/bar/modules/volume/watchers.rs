use std::sync::Arc;

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_audio::{AudioService, core::device::output::OutputDevice};
use wayle_common::{watch, watch_cancellable};
use wayle_config::schemas::modules::VolumeConfig;

use super::{VolumeModule, messages::VolumeCmd};

pub(super) fn spawn_watchers(
    sender: &ComponentSender<VolumeModule>,
    config: &VolumeConfig,
    audio: &Arc<AudioService>,
) {
    let default_output = audio.default_output.clone();
    watch!(sender, [default_output.watch()], |out| {
        let _ = out.send(VolumeCmd::DeviceChanged(default_output.get()));
    });

    let level_icons = config.level_icons.clone();
    let muted_icon = config.icon_muted.clone();
    watch!(sender, [level_icons.watch(), muted_icon.watch()], |out| {
        let _ = out.send(VolumeCmd::IconConfigChanged);
    });
}

pub(super) fn spawn_device_watchers(
    sender: &ComponentSender<VolumeModule>,
    device: &Arc<OutputDevice>,
    token: CancellationToken,
) {
    let volume = device.volume.clone();
    let muted = device.muted.clone();
    watch_cancellable!(sender, token, [volume.watch(), muted.watch()], |out| {
        let _ = out.send(VolumeCmd::VolumeOrMuteChanged);
    });
}
