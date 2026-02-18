mod app_volume_item;
mod messages;
mod methods;
mod watchers;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::{factory::FactoryVecDeque, gtk, prelude::*};
use tracing::warn;
use wayle_audio::{core::stream::AudioStream, volume::types::Volume};
use wayle_common::WatcherToken;
use wayle_config::ConfigService;
use wayle_widgets::prelude::*;

pub(crate) use self::messages::AppVolumesInit;
use self::{
    app_volume_item::{AppVolumeItem, AppVolumeItemOutput},
    messages::{AppVolumesCmd, AppVolumesInput},
};
use crate::i18n::t;

pub(crate) struct AppVolumes {
    config: Arc<ConfigService>,
    playback_streams: Vec<Arc<AudioStream>>,
    app_volumes: FactoryVecDeque<AppVolumeItem>,
    streams_watcher: WatcherToken,
}

#[relm4::component(pub(crate))]
impl Component for AppVolumes {
    type Init = AppVolumesInit;
    type Input = AppVolumesInput;
    type Output = ();
    type CommandOutput = AppVolumesCmd;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_vexpand: true,

            gtk::ScrolledWindow {
                add_css_class: "app-volumes-scroll",
                set_vexpand: true,
                set_hscrollbar_policy: gtk::PolicyType::Never,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    add_css_class: "app-volumes-inner",

                    #[local_ref]
                    app_volume_list -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                    },

                    gtk::Box {
                        #[watch]
                        set_visible: model.playback_streams.is_empty(),
                        set_vexpand: true,
                        set_valign: gtk::Align::Center,

                        #[template]
                        EmptyState {
                            #[template_child]
                            icon {
                                add_css_class: "sm",
                                set_icon_name: Some("ld-volume-x-symbolic"),
                            },
                            #[template_child]
                            title {
                                set_label: &t!("dropdown-audio-no-apps"),
                            },
                        },
                    },
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let playback_streams = init.audio.playback_streams.get();

        let app_volumes = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |item_output| match item_output {
                AppVolumeItemOutput::VolumeChanged(stream_index, percentage) => {
                    AppVolumesInput::AppVolumeChanged(stream_index, percentage)
                }
                AppVolumeItemOutput::ToggleMute(stream_index) => {
                    AppVolumesInput::ToggleAppMute(stream_index)
                }
            });

        watchers::spawn_top_level(&sender, &init.audio, &init.config);

        let mut model = Self {
            config: init.config,
            playback_streams,
            app_volumes,
            streams_watcher: WatcherToken::new(),
        };

        model.sync_app_volumes();
        model.resume_stream_watchers(&sender);

        let app_volume_list = model.app_volumes.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            AppVolumesInput::AppVolumeChanged(stream_index, percentage) => {
                let Some(stream) = self.find_stream(stream_index) else {
                    return;
                };
                let channels = stream.volume.get().channels();
                let volume = Volume::from_percentage(percentage, channels);
                let stream = stream.clone();
                sender.command(|_out, _shutdown| async move {
                    if let Err(err) = stream.set_volume(volume).await {
                        warn!(error = %err, "failed to set app volume");
                    }
                });
            }
            AppVolumesInput::ToggleAppMute(stream_index) => {
                let Some(stream) = self.find_stream(stream_index) else {
                    return;
                };
                let new_muted = !stream.muted.get();
                let stream = stream.clone();
                sender.command(move |_out, _shutdown| async move {
                    if let Err(err) = stream.set_mute(new_muted).await {
                        warn!(error = %err, "failed to toggle app mute");
                    }
                });
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: AppVolumesCmd,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            AppVolumesCmd::PlaybackStreamsChanged(streams) => {
                self.playback_streams = streams;
                self.sync_app_volumes();
                self.resume_stream_watchers(&sender);
            }
            AppVolumesCmd::AppStreamPropertyChanged(stream_index) => {
                self.sync_single_app_volume(stream_index);
            }
            AppVolumesCmd::AppIconSourceChanged => {
                self.sync_app_volumes();
            }
        }
    }
}
