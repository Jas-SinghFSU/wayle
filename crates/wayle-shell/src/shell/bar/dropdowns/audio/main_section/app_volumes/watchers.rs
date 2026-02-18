use std::{sync::Arc, time::Duration};

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_audio::AudioService;
use wayle_common::{watch, watch_cancellable_throttled};
use wayle_config::ConfigService;

use crate::shell::bar::dropdowns::audio::main_section::app_volumes::{
    AppVolumes, messages::AppVolumesCmd,
};

const VOLUME_THROTTLE: Duration = Duration::from_millis(30);

pub(super) fn spawn_top_level(
    sender: &ComponentSender<AppVolumes>,
    audio: &Arc<AudioService>,
    config: &Arc<ConfigService>,
) {
    let playback_streams = audio.playback_streams.clone();
    watch!(sender, [playback_streams.watch()], |out| {
        let _ = out.send(AppVolumesCmd::PlaybackStreamsChanged(
            playback_streams.get(),
        ));
    });

    let app_icon_source = config.config().modules.volume.dropdown_app_icons.clone();
    watch!(sender, [app_icon_source.watch()], |out| {
        let _ = out.send(AppVolumesCmd::AppIconSourceChanged);
    });
}

pub(super) fn spawn_per_stream(
    sender: &ComponentSender<AppVolumes>,
    streams: &[Arc<wayle_audio::core::stream::AudioStream>],
    token: CancellationToken,
) {
    for stream in streams {
        let stream_index = stream.key.index;
        let volume = stream.volume.clone();
        let muted = stream.muted.clone();
        watch_cancellable_throttled!(
            sender,
            token.clone(),
            VOLUME_THROTTLE,
            [volume.watch(), muted.watch()],
            |out| {
                let _ = out.send(AppVolumesCmd::AppStreamPropertyChanged(stream_index));
            }
        );
    }
}
