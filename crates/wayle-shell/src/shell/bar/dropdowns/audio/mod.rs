mod app_volume;
mod device_kind;
mod device_picker;
mod helpers;
mod messages;
mod volume_section;
mod watchers;

use std::{collections::HashSet, sync::Arc};

use gtk::{gdk, prelude::*};
use relm4::{factory::FactoryVecDeque, gtk, prelude::*};
use tracing::warn;
use wayle_audio::{
    AudioService,
    core::{
        device::{input::InputDevice, output::OutputDevice},
        stream::AudioStream,
    },
    volume::types::Volume,
};
use wayle_common::WatcherToken;
use wayle_config::{ConfigService, schemas::modules::AppIconSource};
use wayle_widgets::prelude::*;

use self::{
    app_volume::{AppVolumeInit, AppVolumeItem, AppVolumeItemMsg, AppVolumeItemOutput},
    device_kind::DeviceKind,
    device_picker::{DevicePicker, DevicePickerInit},
    messages::{AudioDropdownCmd, AudioDropdownInit, AudioDropdownMsg},
    volume_section::{VolumeSection, VolumeSectionInit, VolumeSectionInput, VolumeSectionKind},
};
use super::{DropdownFactory, DropdownInstance, DropdownMargins};
use crate::{i18n::t, shell::services::ShellServices};

const BASE_WIDTH: f32 = 382.0;
const BASE_HEIGHT: f32 = 512.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AudioPage {
    Main,
    OutputDevices,
    InputDevices,
}

impl AudioPage {
    fn name(self) -> &'static str {
        match self {
            Self::Main => "main",
            Self::OutputDevices => "output",
            Self::InputDevices => "input",
        }
    }
}

pub(super) struct Factory;

impl DropdownFactory for Factory {
    fn create(services: &ShellServices) -> Option<DropdownInstance> {
        let audio = services.audio.clone()?;
        let config = services.config.clone();

        let init = AudioDropdownInit { audio, config };
        let controller = AudioDropdown::builder().launch(init).detach();

        let popover = controller.widget().clone();
        Some(DropdownInstance::new(popover, Box::new(controller)))
    }
}

pub(crate) struct AudioDropdown {
    pub(super) audio: Arc<AudioService>,
    config: Arc<ConfigService>,
    scaled_width: i32,
    scaled_height: i32,
    margins: DropdownMargins,
    pub(super) active_page: AudioPage,
    has_output: bool,
    has_input: bool,
    has_any_device: bool,

    pub(super) output_section: Controller<VolumeSection>,
    pub(super) input_section: Controller<VolumeSection>,
    pub(super) output_picker: Controller<DevicePicker>,
    pub(super) input_picker: Controller<DevicePicker>,

    pub(super) output_devices: Vec<Arc<OutputDevice>>,
    pub(super) input_devices: Vec<Arc<InputDevice>>,
    pub(super) default_output: Option<Arc<OutputDevice>>,
    pub(super) default_input: Option<Arc<InputDevice>>,

    playback_streams: Vec<Arc<AudioStream>>,
    app_volumes: FactoryVecDeque<AppVolumeItem>,

    pub(super) output_watcher: WatcherToken,
    pub(super) input_watcher: WatcherToken,
    pub(super) streams_watcher: WatcherToken,
}

#[relm4::component(pub(crate))]
impl Component for AudioDropdown {
    type Init = AudioDropdownInit;
    type Input = AudioDropdownMsg;
    type Output = ();
    type CommandOutput = AudioDropdownCmd;

    view! {
        #[root]
        gtk::Popover {
            set_css_classes: &["dropdown", "audio-dropdown"],
            set_has_arrow: false,
            set_autohide: true,
            #[watch]
            set_width_request: model.scaled_width,
            #[watch]
            set_height_request: model.scaled_height,

            #[template]
            Dropdown {
                #[watch]
                set_margin_top: model.margins.top,
                #[watch]
                set_margin_start: model.margins.side,
                #[watch]
                set_margin_end: model.margins.side,
                #[watch]
                set_margin_bottom: model.margins.bottom,

                #[template]
                DropdownHeader {
                    #[template_child]
                    icon {
                        set_visible: true,
                        set_icon_name: Some("ld-volume-2-symbolic"),
                    },
                    #[template_child]
                    label {
                        set_label: &t!("dropdown-audio-title"),
                    },
                },

                #[name = "stack"]
                gtk::Stack {
                    add_css_class: "audio-content",
                    set_vexpand: true,
                    set_transition_type: gtk::StackTransitionType::SlideLeftRight,
                    set_transition_duration: 200,
                    #[watch]
                    set_visible_child_name: model.active_page.name(),

                    add_named[Some("main")] = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        gtk::Box {
                            add_css_class: "audio-fixed",
                            set_orientation: gtk::Orientation::Vertical,
                            #[watch]
                            set_visible: model.has_any_device,

                            #[local_ref]
                            output_section_widget -> gtk::Box {},

                            #[local_ref]
                            input_section_widget -> gtk::Box {},

                            #[template]
                            HorizontalSeparator {
                                add_css_class: "audio-separator",
                            },

                            gtk::Label {
                                add_css_class: "section-label",
                                set_label: &t!("dropdown-audio-app-volume"),
                                set_halign: gtk::Align::Start,
                            },
                        },

                        gtk::ScrolledWindow {
                            add_css_class: "app-volumes-scroll",
                            set_vexpand: true,
                            set_hscrollbar_policy: gtk::PolicyType::Never,
                            #[watch]
                            set_visible: model.has_any_device,

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

                        gtk::Box {
                            #[watch]
                            set_visible: !model.has_any_device,
                            set_vexpand: true,
                            set_valign: gtk::Align::Center,

                            #[template]
                            EmptyState {
                                #[template_child]
                                icon {
                                    set_icon_name: Some("ld-volume-x-symbolic"),
                                },
                                #[template_child]
                                title {
                                    set_label: &t!("dropdown-audio-no-devices-title"),
                                },
                                #[template_child]
                                description {
                                    set_label: &t!("dropdown-audio-no-devices-description"),
                                },
                            },
                        },
                    },

                    #[local_ref]
                    add_named[Some("output")] = output_picker_widget -> gtk::Box {},

                    #[local_ref]
                    add_named[Some("input")] = input_picker_widget -> gtk::Box {},
                },
            },
        }
    }

    #[allow(clippy::too_many_lines)]
    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let output_devices = init.audio.output_devices.get();
        let input_devices = init.audio.input_devices.get();
        let default_output = init.audio.default_output.get();
        let default_input = init.audio.default_input.get();
        let playback_streams = init.audio.playback_streams.get();

        let (output_volume, output_muted) = default_output
            .as_ref()
            .map(|device| (device.volume.get().average_percentage(), device.muted.get()))
            .unwrap_or((0.0, false));

        let (input_volume, input_muted) = default_input
            .as_ref()
            .map(|device| (device.volume.get().average_percentage(), device.muted.get()))
            .unwrap_or((0.0, false));

        let has_output = !output_devices.is_empty();
        let has_input = input_devices.iter().any(|device| !device.is_monitor.get());

        let output_section = VolumeSection::builder()
            .launch(VolumeSectionInit {
                kind: VolumeSectionKind::Output,
                title: t!("dropdown-audio-output"),
                device_name: default_output
                    .as_ref()
                    .map(|device| device.description.get())
                    .unwrap_or_default(),
                device_icon: helpers::output_trigger_icon(&default_output),
                volume: output_volume,
                muted: output_muted,
                has_device: has_output,
            })
            .forward(sender.input_sender(), AudioDropdownMsg::OutputSection);

        let input_section = VolumeSection::builder()
            .launch(VolumeSectionInit {
                kind: VolumeSectionKind::Input,
                title: t!("dropdown-audio-input"),
                device_name: default_input
                    .as_ref()
                    .map(|device| device.description.get())
                    .unwrap_or_default(),
                device_icon: helpers::input_trigger_icon(&default_input),
                volume: input_volume,
                muted: input_muted,
                has_device: has_input,
            })
            .forward(sender.input_sender(), AudioDropdownMsg::InputSection);

        let output_picker = DevicePicker::builder()
            .launch(DevicePickerInit {
                title: t!("dropdown-audio-output-devices"),
            })
            .forward(sender.input_sender(), AudioDropdownMsg::OutputPicker);

        let input_picker = DevicePicker::builder()
            .launch(DevicePickerInit {
                title: t!("dropdown-audio-input-devices"),
            })
            .forward(sender.input_sender(), AudioDropdownMsg::InputPicker);

        let app_volumes = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| match output {
                AppVolumeItemOutput::VolumeChanged(idx, val) => {
                    AudioDropdownMsg::AppVolumeChanged(idx, val)
                }
                AppVolumeItemOutput::ToggleMute(idx) => AudioDropdownMsg::ToggleAppMute(idx),
            });

        let scale = init.config.config().styling.scale.get().value();
        watchers::spawn(&sender, &init.audio, &init.config);

        let mut model = Self {
            audio: init.audio,
            config: init.config,
            scaled_width: Self::scaled_dimension(BASE_WIDTH, scale),
            scaled_height: Self::scaled_dimension(BASE_HEIGHT, scale),
            margins: DropdownMargins::from_scale(scale),
            active_page: AudioPage::Main,
            has_output,
            has_input,
            has_any_device: has_output || has_input,
            output_section,
            input_section,
            output_picker,
            input_picker,
            output_devices,
            input_devices,
            default_output,
            default_input,
            playback_streams,
            app_volumes,
            output_watcher: WatcherToken::new(),
            input_watcher: WatcherToken::new(),
            streams_watcher: WatcherToken::new(),
        };

        model.sync_app_volumes();
        OutputDevice::send_devices(&model);
        InputDevice::send_devices(&model);
        model.resume_app_stream_watchers(&sender);
        OutputDevice::resume_watcher(&mut model, &sender);
        InputDevice::resume_watcher(&mut model, &sender);

        let output_section_widget = model.output_section.widget();
        let input_section_widget = model.input_section.widget();
        let output_picker_widget = model.output_picker.widget();
        let input_picker_widget = model.input_picker.widget();
        let app_volume_list = model.app_volumes.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            AudioDropdownMsg::OutputSection(output) => {
                self.handle_volume_output::<OutputDevice>(output, &sender);
            }
            AudioDropdownMsg::InputSection(output) => {
                self.handle_volume_output::<InputDevice>(output, &sender);
            }
            AudioDropdownMsg::OutputPicker(output) => {
                self.handle_picker_output::<OutputDevice>(output, &sender);
            }
            AudioDropdownMsg::InputPicker(output) => {
                self.handle_picker_output::<InputDevice>(output, &sender);
            }
            AudioDropdownMsg::AppVolumeChanged(stream_index, percentage) => {
                if let Some(stream) = self
                    .playback_streams
                    .iter()
                    .find(|playback_stream| playback_stream.key.index == stream_index)
                {
                    let channels = stream.volume.get().channels();
                    let volume = Volume::from_percentage(percentage, channels);
                    let stream = stream.clone();
                    sender.command(|_out, _shutdown| async move {
                        if let Err(err) = stream.set_volume(volume).await {
                            warn!(error = %err, "failed to set app volume");
                        }
                    });
                }
            }
            AudioDropdownMsg::ToggleAppMute(stream_index) => {
                if let Some(stream) = self
                    .playback_streams
                    .iter()
                    .find(|playback_stream| playback_stream.key.index == stream_index)
                {
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
    }

    fn update_cmd(
        &mut self,
        msg: AudioDropdownCmd,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            AudioDropdownCmd::DefaultOutputChanged(device) => {
                self.handle_default_output_changed(device, &sender);
            }
            AudioDropdownCmd::DefaultInputChanged(device) => {
                self.handle_default_input_changed(device, &sender);
            }
            AudioDropdownCmd::OutputVolumeOrMuteChanged => {
                if let Some(ref device) = self.default_output {
                    self.output_section.emit(VolumeSectionInput::SetVolume(
                        device.volume.get().average_percentage(),
                    ));
                    self.output_section
                        .emit(VolumeSectionInput::SetMuted(device.muted.get()));
                }
            }
            AudioDropdownCmd::InputVolumeOrMuteChanged => {
                if let Some(ref device) = self.default_input {
                    self.input_section.emit(VolumeSectionInput::SetVolume(
                        device.volume.get().average_percentage(),
                    ));
                    self.input_section
                        .emit(VolumeSectionInput::SetMuted(device.muted.get()));
                }
            }
            AudioDropdownCmd::OutputDevicesChanged(devices) => {
                self.output_devices = devices;
                self.has_output = !self.output_devices.is_empty();
                self.has_any_device = self.has_output || self.has_input;
                self.output_section
                    .emit(VolumeSectionInput::SetHasDevice(self.has_output));
                OutputDevice::send_devices(self);
            }
            AudioDropdownCmd::InputDevicesChanged(devices) => {
                self.input_devices = devices;
                self.has_input = self
                    .input_devices
                    .iter()
                    .any(|device| !device.is_monitor.get());
                self.has_any_device = self.has_output || self.has_input;
                self.input_section
                    .emit(VolumeSectionInput::SetHasDevice(self.has_input));
                InputDevice::send_devices(self);
            }
            AudioDropdownCmd::PlaybackStreamsChanged(streams) => {
                self.playback_streams = streams;
                self.sync_app_volumes();
                self.resume_app_stream_watchers(&sender);
            }
            AudioDropdownCmd::AppStreamPropertyChanged(stream_index) => {
                self.sync_single_app_volume(stream_index);
            }
            AudioDropdownCmd::ScaleChanged(scale) => {
                self.scaled_width = Self::scaled_dimension(BASE_WIDTH, scale);
                self.scaled_height = Self::scaled_dimension(BASE_HEIGHT, scale);
                self.margins = DropdownMargins::from_scale(scale);
            }
            AudioDropdownCmd::AppIconSourceChanged => {
                self.sync_app_volumes();
            }
        }
    }
}

impl AudioDropdown {
    pub(super) fn resume_app_stream_watchers(&mut self, sender: &ComponentSender<Self>) {
        let token = self.streams_watcher.reset();
        watchers::spawn_playback_streams(sender, &self.playback_streams, token);
    }

    fn scaled_dimension(base: f32, scale: f32) -> i32 {
        (base * scale).round() as i32
    }

    fn resolve_stream_icon(
        props: &std::collections::HashMap<String, String>,
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

    fn sync_app_volumes(&mut self) {
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

        items.sort_by(|a, b| a.name.cmp(&b.name));

        let mut guard = self.app_volumes.guard();
        guard.clear();
        for init in items {
            guard.push_back(init);
        }
    }

    fn sync_single_app_volume(&mut self, stream_index: u32) {
        let Some(stream) = self
            .playback_streams
            .iter()
            .find(|playback_stream| playback_stream.key.index == stream_index)
        else {
            return;
        };

        let item_index = {
            let guard = self.app_volumes.guard();
            guard
                .iter()
                .position(|item| item.stream_index == stream_index)
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
