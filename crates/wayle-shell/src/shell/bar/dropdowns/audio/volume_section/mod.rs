mod messages;

use gtk::{gdk, glib, pango, prelude::*};
use relm4::{gtk, prelude::*};
use wayle_widgets::prelude::GhostIconButton;

pub(super) use self::messages::*;
use super::helpers;
use crate::i18n::t;

pub(crate) struct VolumeSection {
    kind: VolumeSectionKind,
    title: String,
    device_name: String,
    device_icon: &'static str,
    volume: f64,
    muted: bool,
    has_device: bool,
    dragging: bool,
}

impl VolumeSection {
    fn mute_icon(&self) -> &'static str {
        match self.kind {
            VolumeSectionKind::Output => helpers::volume_icon(self.volume, self.muted),
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
                            set_ellipsize: pango::EllipsizeMode::End,
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

                #[name = "slider"]
                gtk::Scale {
                    add_css_class: "audio-volume-slider",
                    set_draw_value: false,
                    set_cursor_from_name: Some("pointer"),
                    set_has_origin: true,
                    set_hexpand: true,
                    set_range: (0.0, 100.0),
                    #[watch]
                    #[block_signal(vol_handler)]
                    set_value: model.volume,
                    connect_value_changed[sender] => move |scale| {
                        sender.input(VolumeSectionInput::SliderMoved(scale.value()));
                    } @vol_handler,
                },

                gtk::Label {
                    add_css_class: "audio-slider-value",
                    set_width_chars: 4,
                    set_xalign: 1.0,
                    #[watch]
                    set_label: &helpers::format_volume(model.volume),
                },
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
        let model = VolumeSection {
            kind: init.kind,
            title: init.title,
            device_name: init.device_name,
            device_icon: init.device_icon,
            volume: init.volume,
            muted: init.muted,
            has_device: init.has_device,
            dragging: false,
        };

        let widgets = view_output!();

        let slider_ref = widgets.slider.clone();
        let drag_sender = sender.input_sender().clone();
        let controller = gtk::EventControllerLegacy::new();
        controller.connect_event(move |_, event| {
            match event.event_type() {
                gdk::EventType::ButtonPress | gdk::EventType::TouchBegin => {
                    drag_sender.emit(VolumeSectionInput::DragStarted);
                }
                gdk::EventType::ButtonRelease
                | gdk::EventType::TouchEnd
                | gdk::EventType::TouchCancel => {
                    drag_sender.emit(VolumeSectionInput::SliderMoved(slider_ref.value()));
                    drag_sender.emit(VolumeSectionInput::DragEnded);
                }
                _ => {}
            }
            glib::Propagation::Proceed
        });
        widgets.slider.add_controller(controller);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            VolumeSectionInput::SliderMoved(volume) => {
                self.volume = volume;
                let _ = sender.output(VolumeSectionOutput::VolumeChanged(volume));
            }
            VolumeSectionInput::DragStarted => {
                self.dragging = true;
            }
            VolumeSectionInput::DragEnded => {
                self.dragging = false;
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
                self.muted = muted;
                if !self.dragging {
                    self.volume = volume;
                }
            }
            VolumeSectionInput::SetVolume(volume) => {
                if !self.dragging {
                    self.volume = volume;
                }
            }
            VolumeSectionInput::SetMuted(muted) => {
                if !self.dragging {
                    self.muted = muted;
                }
            }
            VolumeSectionInput::SetHasDevice(has_device) => {
                self.has_device = has_device;
            }
        }
    }
}
