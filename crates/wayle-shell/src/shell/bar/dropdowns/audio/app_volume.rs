use gtk::{glib, pango, prelude::*};
use relm4::{gtk, prelude::*};
use wayle_widgets::prelude::{DebouncedSlider, GhostIconButton};

use super::helpers;

pub(super) struct AppVolumeInit {
    pub name: String,
    pub icon: Option<String>,
    pub volume: f64,
    pub muted: bool,
    pub stream_index: u32,
}

pub(super) struct AppVolumeItem {
    pub name: String,
    pub icon: Option<String>,
    pub muted: bool,
    pub stream_index: u32,
    slider: DebouncedSlider,
}

#[derive(Debug)]
pub(super) enum AppVolumeItemMsg {
    SetBackendState { volume: f64, muted: bool },
    VolumeCommitted(f64),
    ToggleMute,
}

#[derive(Debug)]
pub(super) enum AppVolumeItemOutput {
    VolumeChanged(u32, f64),
    ToggleMute(u32),
}

#[relm4::factory(pub(super))]
impl FactoryComponent for AppVolumeItem {
    type Init = AppVolumeInit;
    type Input = AppVolumeItemMsg;
    type Output = AppVolumeItemOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            add_css_class: "audio-app-item",
            set_orientation: gtk::Orientation::Vertical,

            gtk::Box {
                add_css_class: "audio-app-header",

                gtk::Box {
                    add_css_class: "audio-app-icon",
                    set_valign: gtk::Align::Center,

                    gtk::Image {
                        #[watch]
                        set_icon_name: Some(self.icon.as_deref().unwrap_or("ld-app-window-symbolic")),
                    },
                },

                gtk::Label {
                    add_css_class: "audio-app-name",
                    set_hexpand: true,
                    set_halign: gtk::Align::Start,
                    set_ellipsize: pango::EllipsizeMode::End,
                    #[watch]
                    set_label: &self.name,
                },
            },

            gtk::Box {
                add_css_class: "audio-app-controls",

                #[template]
                GhostIconButton {
                    add_css_class: "audio-app-mute",
                    #[watch]
                    set_icon_name: helpers::volume_icon(self.slider.value(), self.muted),
                    connect_clicked => AppVolumeItemMsg::ToggleMute,
                },

                #[local_ref]
                slider_widget -> gtk::Box {},
            },
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        Self {
            name: init.name,
            icon: init.icon,
            muted: init.muted,
            stream_index: init.stream_index,
            slider: DebouncedSlider::with_label(init.volume),
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        _root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        if let Some(scale) = self.slider.scale() {
            scale.add_css_class("audio-app-slider");
        }
        if let Some(label) = self.slider.label_widget() {
            label.add_css_class("audio-app-value");
        }

        let commit_sender = sender.input_sender().clone();
        self.slider.connect_closure(
            "committed",
            false,
            glib::closure_local!(move |_widget: DebouncedSlider, value: f64| {
                commit_sender.emit(AppVolumeItemMsg::VolumeCommitted(value));
            }),
        );

        let slider_widget = self.slider.upcast_ref::<gtk::Box>();
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, msg: Self::Input, sender: FactorySender<Self>) {
        match msg {
            AppVolumeItemMsg::SetBackendState { volume, muted } => {
                self.slider.set_value(volume);
                self.muted = muted;
            }
            AppVolumeItemMsg::VolumeCommitted(volume) => {
                let _ = sender.output(AppVolumeItemOutput::VolumeChanged(
                    self.stream_index,
                    volume,
                ));
            }
            AppVolumeItemMsg::ToggleMute => {
                let _ = sender.output(AppVolumeItemOutput::ToggleMute(self.stream_index));
            }
        }
    }
}
