use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use gtk::gdk;
use relm4::{gtk, prelude::*};
use wayle_audio::core::stream::AudioStream;
use wayle_config::schemas::modules::AppIconSource;

use crate::shell::bar::dropdowns::audio::{
    helpers,
    main_section::app_volumes::{
        AppVolumes,
        app_volume_item::{AppVolumeInit, AppVolumeItemMsg},
    },
};

impl AppVolumes {
    pub(super) fn find_stream(&self, stream_index: u32) -> Option<&Arc<AudioStream>> {
        self.playback_streams
            .iter()
            .find(|stream| stream.key.index == stream_index)
    }

    pub(crate) fn resume_stream_watchers(&mut self, sender: &ComponentSender<Self>) {
        let token = self.streams_watcher.reset();
        super::watchers::spawn_per_stream(sender, &self.playback_streams, token);
    }

    fn resolve_stream_icon(
        props: &HashMap<String, String>,
        icon_source: AppIconSource,
    ) -> Option<String> {
        let icon = helpers::stream_icon(props, icon_source);

        if icon_source == AppIconSource::Native {
            let theme = gtk::IconTheme::for_display(&gdk::Display::default()?);
            if icon.as_ref().is_some_and(|name| theme.has_icon(name)) {
                return icon;
            }
            return helpers::stream_icon(props, AppIconSource::Mapped);
        }

        icon
    }

    pub(crate) fn sync_app_volumes(&mut self) {
        let icon_source = self.config.config().modules.volume.dropdown_app_icons.get();
        let mut seen_pids: HashSet<u32> = HashSet::new();

        let mut items: Vec<AppVolumeInit> = self
            .playback_streams
            .iter()
            .filter_map(|stream| {
                let props = stream.properties.get();

                if helpers::is_event_stream(&props) {
                    return None;
                }

                if let Some(pid) = stream.pid.get()
                    && !seen_pids.insert(pid)
                {
                    return None;
                }

                let name =
                    helpers::app_display_name(&stream.application_name.get(), &stream.name.get());
                let icon = Self::resolve_stream_icon(&props, icon_source);
                let volume = stream.volume.get().average_percentage();
                let muted = stream.muted.get();

                Some(AppVolumeInit {
                    name,
                    icon,
                    volume,
                    muted,
                    stream_index: stream.key.index,
                })
            })
            .collect();

        items.sort_by(|left, right| left.name.cmp(&right.name));

        let mut guard = self.app_volumes.guard();
        guard.clear();
        for init in items {
            guard.push_back(init);
        }
    }

    pub(crate) fn sync_single_app_volume(&mut self, stream_index: u32) {
        let Some(stream) = self
            .playback_streams
            .iter()
            .find(|stream| stream.key.index == stream_index)
        else {
            return;
        };

        let item_index = {
            let guard = self.app_volumes.guard();
            guard
                .iter()
                .position(|volume_item| volume_item.stream_index == stream_index)
        };

        if let Some(item_index) = item_index {
            self.app_volumes.send(
                item_index,
                AppVolumeItemMsg::SetBackendState {
                    volume: stream.volume.get().average_percentage(),
                    muted: stream.muted.get(),
                },
            );
        }
    }
}
