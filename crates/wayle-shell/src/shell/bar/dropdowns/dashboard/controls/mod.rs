mod messages;
mod watchers;

use std::sync::Arc;

use gtk::{glib, prelude::*};
use relm4::{gtk, prelude::*};
use wayle_audio::{AudioService, volume::types::Volume};
use wayle_widgets::{
    WatcherToken,
    prelude::{DebouncedSlider, GhostIconButton},
};

pub(crate) use self::messages::ControlsInit;
use self::messages::{ControlsCmd, ControlsInput};
use crate::i18n::t;

pub(crate) struct ControlsSection {
    audio: Option<Arc<AudioService>>,
    has_device: bool,
    muted: bool,
    device_name: String,
    slider: DebouncedSlider,
    device_watcher: WatcherToken,
}

#[relm4::component(pub(crate))]
impl Component for ControlsSection {
    type Init = ControlsInit;
    type Input = ControlsInput;
    type Output = ();
    type CommandOutput = ControlsCmd;

    view! {
        #[root]
        gtk::Box {
            set_css_classes: &["card", "dashboard-card"],
            set_orientation: gtk::Orientation::Vertical,

            #[name = "header"]
            gtk::Box {
                add_css_class: "card-header",

                #[name = "card_title"]
                gtk::Box {
                    add_css_class: "card-title",

                    gtk::Image {
                        set_icon_name: Some("ld-audio-lines-symbolic"),
                    },

                    gtk::Label {
                        set_label: &t!("dropdown-dashboard-volume"),
                    },
                },
            },

            #[name = "controls_container"]
            gtk::Box {
                add_css_class: "dashboard-controls",
                set_orientation: gtk::Orientation::Vertical,

                #[name = "slider_row"]
                gtk::Box {
                    add_css_class: "dashboard-slider-row",
                    #[watch]
                    set_sensitive: model.has_device,

                    #[template]
                    #[name = "mute_btn"]
                    GhostIconButton {
                        add_css_class: "dashboard-volume-icon",
                        connect_clicked => ControlsInput::MuteToggled,

                        gtk::Image {
                            #[watch]
                            set_icon_name: Some(if model.muted || !model.has_device {
                                "ld-volume-x-symbolic"
                            } else {
                                "ld-volume-2-symbolic"
                            }),
                        },
                    },

                    #[local_ref]
                    slider_widget -> gtk::Box {
                        set_hexpand: true,
                    },
                },

                #[name = "device_label"]
                gtk::Label {
                    add_css_class: "controls-device",
                    set_halign: gtk::Align::End,
                    set_ellipsize: gtk::pango::EllipsizeMode::End,
                    #[watch]
                    set_label: &model.device_name,
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let slider = DebouncedSlider::with_label(0.0);

        if let Some(scale) = slider.scale() {
            scale.add_css_class("dashboard-volume-slider");
        }
        if let Some(label) = slider.label_widget() {
            label.add_css_class("dashboard-slider-value");
        }

        let commit_sender = sender.input_sender().clone();
        slider.connect_closure(
            "committed",
            false,
            glib::closure_local!(move |_slider: DebouncedSlider, percentage: f64| {
                commit_sender.emit(ControlsInput::VolumeCommitted(percentage));
            }),
        );

        watchers::spawn(&sender, &init.audio);

        let model = Self {
            audio: init.audio,
            has_device: false,
            muted: false,
            device_name: t!("dropdown-dashboard-no-device"),
            slider: slider.clone(),
            device_watcher: WatcherToken::new(),
        };

        let slider_widget = model.slider.upcast_ref::<gtk::Box>();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            ControlsInput::VolumeCommitted(percentage) => {
                let Some(audio) = self.audio.clone() else {
                    return;
                };

                sender.oneshot_command(async move {
                    if let Some(device) = audio.default_output.get() {
                        let channels = device.volume.get().channels();
                        let volume = Volume::from_percentage(percentage, channels);
                        if let Err(err) = device.set_volume(volume).await {
                            tracing::warn!(error = %err, "volume set failed");
                        }
                    }
                    ControlsCmd::VolumeChanged(percentage)
                });
            }
            ControlsInput::MuteToggled => {
                let Some(audio) = self.audio.clone() else {
                    return;
                };

                let target = !self.muted;

                sender.oneshot_command(async move {
                    if let Some(device) = audio.default_output.get()
                        && let Err(err) = device.set_mute(target).await
                    {
                        tracing::warn!(error = %err, "mute toggle failed");
                    }
                    ControlsCmd::MuteChanged(target)
                });
            }
        }
    }

    fn update_cmd(&mut self, msg: ControlsCmd, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            ControlsCmd::VolumeChanged(pct) => {
                self.slider.set_property("value", pct);
            }
            ControlsCmd::MuteChanged(muted) => {
                self.muted = muted;
            }
            ControlsCmd::DeviceNameChanged(name) => {
                self.device_name = name;
            }
            ControlsCmd::DeviceAvailable(available) => {
                if available && let Some(audio) = &self.audio {
                    let token = self.device_watcher.reset();
                    watchers::spawn_device_watchers(&sender, audio, token);
                } else if !available {
                    self.device_name = t!("dropdown-dashboard-no-device");
                }

                self.has_device = available;
            }
        }
    }
}
