mod messages;

use gtk::{glib, prelude::*};
use relm4::{gtk, prelude::*};
use wayle_widgets::prelude::{DebouncedSlider, GhostIconButton};

pub(super) use self::messages::*;
use super::helpers;
use crate::i18n::t;

pub(crate) struct VolumeSection {
    kind: VolumeSectionKind,
    title: String,
    device_name: String,
    device_icon: &'static str,
    muted: bool,
    has_device: bool,
    slider: DebouncedSlider,
}

impl VolumeSection {
    fn mute_icon(&self) -> &'static str {
        match self.kind {
            VolumeSectionKind::Output => helpers::volume_icon(self.slider.value(), self.muted),
            VolumeSectionKind::Input => helpers::input_icon(self.muted),
        }
    }
}

#[relm4::component(pub(crate))]
impl SimpleComponent for VolumeSection {
    type Init = VolumeSectionInit;
    type Input = VolumeSectionInput;
    type Output = VolumeSectionOutput;

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
        let slider = DebouncedSlider::with_label(init.volume);

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
            glib::closure_local!(move |_widget: DebouncedSlider, value: f64| {
                commit_sender.emit(VolumeSectionInput::VolumeCommitted(value));
            }),
        );

        let model = VolumeSection {
            kind: init.kind,
            title: init.title,
            device_name: init.device_name,
            device_icon: init.device_icon,
            muted: init.muted,
            has_device: init.has_device,
            slider,
        };

        let slider_widget = model.slider.upcast_ref::<gtk::Box>();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            VolumeSectionInput::VolumeCommitted(volume) => {
                let _ = sender.output(VolumeSectionOutput::VolumeChanged(volume));
            }
            VolumeSectionInput::MuteClicked => {
                let _ = sender.output(VolumeSectionOutput::ToggleMute);
            }
            VolumeSectionInput::ShowDevicesClicked => {
                let _ = sender.output(VolumeSectionOutput::ShowDevices);
            }
            VolumeSectionInput::SetDevice {
                name,
                icon,
                volume,
                muted,
            } => {
                self.device_name = name;
                self.device_icon = icon;
                self.slider.set_value(volume);
                self.muted = muted;
            }
            VolumeSectionInput::SetVolume(volume) => {
                self.slider.set_value(volume);
            }
            VolumeSectionInput::SetMuted(muted) => {
                self.muted = muted;
            }
            VolumeSectionInput::SetHasDevice(has_device) => {
                self.has_device = has_device;
            }
        }
    }
}
