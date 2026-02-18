mod messages;
mod methods;
mod watchers;

use std::sync::Arc;

use gtk::{glib, prelude::*};
use relm4::{gtk, prelude::*};
use tracing::warn;
use wayle_audio::{AudioService, volume::types::Volume};
use wayle_common::WatcherToken;
use wayle_widgets::prelude::{DebouncedSlider, GhostIconButton};

pub(crate) use self::messages::*;
use crate::i18n::t;

pub(crate) struct VolumeSection {
    audio: Arc<AudioService>,
    kind: VolumeSectionKind,
    title: String,
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
            add_css_class: "audio-section",
            set_orientation: gtk::Orientation::Vertical,

            gtk::Box {
                add_css_class: "audio-section-header",

                gtk::Label {
                    add_css_class: "audio-section-title",
                    #[watch]
                    set_label: &model.title,
                    set_hexpand: true,
                    set_halign: gtk::Align::Start,
                },

                gtk::Button {
                    add_css_class: "device-trigger",
                    set_cursor_from_name: Some("pointer"),
                    #[watch]
                    set_visible: model.has_device,
                    connect_clicked => VolumeSectionInput::ShowDevicesClicked,

                    gtk::Box {
                        gtk::Image {
                            add_css_class: "device-trigger-icon",
                            #[watch]
                            set_icon_name: Some(model.device_icon),
                        },
                        gtk::Label {
                            add_css_class: "device-trigger-name",
                            set_ellipsize: gtk::pango::EllipsizeMode::End,
                            set_max_width_chars: 20,
                            #[watch]
                            set_label: &model.device_name,
                        },
                        gtk::Image {
                            add_css_class: "device-trigger-chevron",
                            set_icon_name: Some("ld-chevron-right-symbolic"),
                        },
                    },
                },
            },

            gtk::Box {
                add_css_class: "audio-slider-row",
                #[watch]
                set_visible: model.has_device,

                #[template]
                GhostIconButton {
                    add_css_class: "audio-slider-icon",
                    #[watch]
                    set_icon_name: model.mute_icon(),
                    connect_clicked => VolumeSectionInput::MuteClicked,
                },

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
            title: init.title,
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
                if let Some(ref device) = self.device {
                    let channels = device.channels();
                    let volume = Volume::from_percentage(percentage, channels);
                    let device = device.clone();
                    sender.command(|_out, _shutdown| async move {
                        if let Err(err) = device.set_volume(volume).await {
                            warn!(error = %err, "failed to set volume");
                        }
                    });
                }
            }
            VolumeSectionInput::MuteClicked => {
                if let Some(ref device) = self.device {
                    let new_muted = !device.muted();
                    let device = device.clone();
                    sender.oneshot_command(async move {
                        if let Err(err) = device.set_mute(new_muted).await {
                            warn!(error = %err, "failed to toggle mute");
                        }
                        VolumeSectionCmd::VolumeOrMuteChanged
                    });
                }
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
