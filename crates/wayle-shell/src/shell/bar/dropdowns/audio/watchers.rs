use std::sync::Arc;

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_audio::AudioService;
use wayle_common::{watch, watch_cancellable};
use wayle_config::ConfigService;

use super::{AudioDropdown, messages::AudioDropdownCmd};

pub(super) fn spawn(
    sender: &ComponentSender<AudioDropdown>,
    audio: &Arc<AudioService>,
    config: &Arc<ConfigService>,
) {
    let scale = config.config().styling.scale.clone();
    watch!(sender, [scale.watch()], |out| {
        let _ = out.send(AudioDropdownCmd::ScaleChanged(scale.get().value()));
    });
    let default_output = audio.default_output.clone();
    watch!(sender, [default_output.watch()], |out| {
        let _ = out.send(AudioDropdownCmd::DefaultOutputChanged(default_output.get()));
    });

    let default_input = audio.default_input.clone();
    watch!(sender, [default_input.watch()], |out| {
        let _ = out.send(AudioDropdownCmd::DefaultInputChanged(default_input.get()));
    });

    let output_devices = audio.output_devices.clone();
    watch!(sender, [output_devices.watch()], |out| {
        let _ = out.send(AudioDropdownCmd::OutputDevicesChanged(output_devices.get()));
    });

    let input_devices = audio.input_devices.clone();
    watch!(sender, [input_devices.watch()], |out| {
        let _ = out.send(AudioDropdownCmd::InputDevicesChanged(input_devices.get()));
    });

    let playback_streams = audio.playback_streams.clone();
    watch!(sender, [playback_streams.watch()], |out| {
        let _ = out.send(AudioDropdownCmd::PlaybackStreamsChanged(
            playback_streams.get(),
        ));
    });

    let app_icon_source = config.config().modules.volume.dropdown_app_icons.clone();
    watch!(sender, [app_icon_source.watch()], |out| {
        let _ = out.send(AudioDropdownCmd::AppIconSourceChanged);
    });
}

pub(super) fn spawn_output_device(
    sender: &ComponentSender<AudioDropdown>,
    device: &Arc<wayle_audio::core::device::output::OutputDevice>,
    token: CancellationToken,
) {
    let volume = device.volume.clone();
    let muted = device.muted.clone();
    watch_cancellable!(sender, token, [volume.watch(), muted.watch()], |out| {
        let _ = out.send(AudioDropdownCmd::OutputVolumeOrMuteChanged);
    });
}

pub(super) fn spawn_input_device(
    sender: &ComponentSender<AudioDropdown>,
    device: &Arc<wayle_audio::core::device::input::InputDevice>,
    token: CancellationToken,
) {
    let volume = device.volume.clone();
    let muted = device.muted.clone();
    watch_cancellable!(sender, token, [volume.watch(), muted.watch()], |out| {
        let _ = out.send(AudioDropdownCmd::InputVolumeOrMuteChanged);
    });
}

pub(super) fn spawn_playback_streams(
    sender: &ComponentSender<AudioDropdown>,
    streams: &[Arc<wayle_audio::core::stream::AudioStream>],
    token: CancellationToken,
) {
    for stream in streams {
        let stream_index = stream.key.index;
        let volume = stream.volume.clone();
        let muted = stream.muted.clone();
        watch_cancellable!(
            sender,
            token.clone(),
            [volume.watch(), muted.watch()],
            |out| {
                let _ = out.send(AudioDropdownCmd::AppStreamPropertyChanged(stream_index));
            }
        );
    }
}
