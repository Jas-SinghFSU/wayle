mod messages;
mod methods;
mod watchers;

use std::sync::Arc;

use gtk::{glib, prelude::*};
use relm4::{gtk, prelude::*};
use wayle_audio::AudioService;
use wayle_common::WatcherToken;
use wayle_widgets::prelude::DebouncedSlider;

pub(crate) use self::messages::*;
use crate::i18n::t;

pub(crate) struct VolumeSection {
    audio: Arc<AudioService>,
    kind: VolumeSectionKind,
    device: Option<ActiveDevice>,
    device_name: String,
    device_icon: &'static str,
    muted: bool,
    has_device: bool,
    slider: DebouncedSlider,
    device_watcher: WatcherToken,
}

#[relm4::component(pub(crate))]
impl Component for VolumeSection {
    type Init = VolumeSectionInit;
    type Input = VolumeSectionInput;
    type Output = VolumeSectionOutput;
    type CommandOutput = VolumeSectionCmd;

    view! {
        #[root]
        gtk::Box {
            add_css_class: "audio-device",
            set_orientation: gtk::Orientation::Vertical,
            #[watch]
            set_class_active: ("muted", model.muted),

            gtk::Box {
                add_css_class: "audio-device-trigger",
                #[watch]
                set_visible: model.has_device,

                gtk::Box {
                    add_css_class: "audio-device-icon",
                    set_valign: gtk::Align::Center,

                    gtk::Image {
                        add_css_class: "audio-device-icon-img",
                        #[watch]
                        set_icon_name: Some(model.device_icon),
                    },
                },

                gtk::Box {
                    add_css_class: "audio-device-info",
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,

                    gtk::Label {
                        add_css_class: "audio-device-label",
                        set_halign: gtk::Align::Start,
                        #[watch]
                        set_label: &model.label(),
                    },

                    gtk::Button {
                        add_css_class: "audio-device-trigger-btn",
                        set_cursor_from_name: Some("pointer"),
                        set_halign: gtk::Align::Start,
                        connect_clicked => VolumeSectionInput::ShowDevicesClicked,

                        gtk::Box {
                            add_css_class: "audio-device-name",

                            gtk::Label {
                                add_css_class: "audio-device-name-text",
                                set_ellipsize: gtk::pango::EllipsizeMode::End,
                                #[watch]
                                set_label: &model.device_name,
                            },

                            gtk::Image {
                                add_css_class: "audio-device-chevron",
                                set_icon_name: Some("ld-chevron-right-symbolic"),
                            },
                        },
                    },
                },

                gtk::Button {
                    add_css_class: "audio-mute-btn",
                    set_valign: gtk::Align::Center,
                    set_cursor_from_name: Some("pointer"),
                    #[watch]
                    set_class_active: ("muted", model.muted),
                    connect_clicked => VolumeSectionInput::MuteClicked,

                    gtk::Image {
                        add_css_class: "audio-mute-icon",
                        #[watch]
                        set_icon_name: Some(model.mute_icon()),
                    },
                },
            },

            gtk::Box {
                add_css_class: "audio-slider-row",
                #[watch]
                set_visible: model.has_device,

                #[local_ref]
                slider_widget -> gtk::Box {},
            },

            gtk::Box {
                add_css_class: "audio-no-device",
                set_halign: gtk::Align::Center,
                #[watch]
                set_visible: !model.has_device,

                gtk::Image {
                    add_css_class: "audio-no-device-icon",
                    #[watch]
                    set_icon_name: Some("tb-alert-triangle-symbolic"),
                },
                gtk::Label {
                    add_css_class: "audio-no-device-label",
                    #[watch]
                    set_label: &t!("dropdown-audio-no-device"),
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let default_device = match init.kind {
            VolumeSectionKind::Output => init.audio.default_output.get().map(ActiveDevice::Output),
            VolumeSectionKind::Input => init
                .audio
                .default_input
                .get()
                .filter(|device| !device.is_monitor.get())
                .map(ActiveDevice::Input),
        };

        let (device_name, device_icon, volume, muted) = default_device
            .as_ref()
            .map(|device| {
                (
                    device.description(),
                    device.trigger_icon(),
                    device.volume_percentage(),
                    device.muted(),
                )
            })
            .unwrap_or_default();

        let has_device = match init.kind {
            VolumeSectionKind::Output => !init.audio.output_devices.get().is_empty(),
            VolumeSectionKind::Input => init
                .audio
                .input_devices
                .get()
                .iter()
                .any(|device| !device.is_monitor.get()),
        };

        let slider = DebouncedSlider::with_label(volume);
        if let Some(scale) = slider.scale() {
            scale.add_css_class("audio-volume-slider");
        }
        if let Some(label) = slider.label_widget() {
            label.add_css_class("audio-slider-value");
        }

        let commit_sender = sender.input_sender().clone();
        slider.connect_closure(
            "committed",
            false,
            glib::closure_local!(move |_slider: DebouncedSlider, percentage: f64| {
                commit_sender.emit(VolumeSectionInput::VolumeCommitted(percentage));
            }),
        );

        watchers::spawn_default_device(&sender, &init.audio, init.kind);

        let mut model = Self {
            audio: init.audio,
            kind: init.kind,
            device: default_device,
            device_name,
            device_icon,
            muted,
            has_device,
            slider,
            device_watcher: WatcherToken::new(),
        };

        model.resume_device_watcher(&sender);

        let _ = sender.output(VolumeSectionOutput::HasDeviceChanged(has_device));

        let slider_widget = model.slider.upcast_ref::<gtk::Box>();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            VolumeSectionInput::VolumeCommitted(percentage) => {
                self.commit_volume(percentage, &sender);
            }
            VolumeSectionInput::MuteClicked => {
                self.toggle_mute(&sender);
            }
            VolumeSectionInput::ShowDevicesClicked => {
                let _ = sender.output(VolumeSectionOutput::ShowDevices);
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: Self::CommandOutput,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            VolumeSectionCmd::DeviceChanged(device) => {
                let device = device.or_else(|| self.current_default());

                let had_device = self.has_device;
                self.has_device = self.check_has_device();

                if let Some(ref device) = device {
                    self.sync_from_device(device);
                }
                self.device = device;

                self.resume_device_watcher(&sender);

                if self.has_device != had_device {
                    let _ = sender.output(VolumeSectionOutput::HasDeviceChanged(self.has_device));
                }
            }
            VolumeSectionCmd::VolumeOrMuteChanged => {
                if let Some(ref device) = self.device {
                    self.slider.set_value(device.volume_percentage());
                    self.muted = device.muted();
                }
            }
        }
    }
}
