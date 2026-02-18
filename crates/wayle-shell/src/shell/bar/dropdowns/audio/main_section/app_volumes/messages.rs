use std::sync::Arc;

use wayle_audio::{AudioService, core::stream::AudioStream};
use wayle_config::ConfigService;

pub(crate) struct AppVolumesInit {
    pub audio: Arc<AudioService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum AppVolumesInput {
    AppVolumeChanged(u32, f64),
    ToggleAppMute(u32),
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum AppVolumesCmd {
    PlaybackStreamsChanged(Vec<Arc<AudioStream>>),
    AppStreamPropertyChanged(u32),
    AppIconSourceChanged,
}
