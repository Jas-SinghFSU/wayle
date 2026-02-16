use gtk::{gdk, glib, pango, prelude::*};
use relm4::{gtk, prelude::*};
use wayle_widgets::prelude::{GhostIconButton, Slider};

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
    pub volume: f64,
    pub muted: bool,
    pub stream_index: u32,
    pub dragging: bool,
}

#[derive(Debug)]
pub(super) enum AppVolumeItemMsg {
    SetBackendState { volume: f64, muted: bool },
    SliderMoved(f64),
    DragStarted,
    DragEnded,
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
                    set_icon_name: helpers::volume_icon(self.volume, self.muted),
                    connect_clicked => AppVolumeItemMsg::ToggleMute,
                },

                #[template]
                #[name = "volume_slider"]
                Slider {
                    add_css_class: "audio-app-slider",
                    set_hexpand: true,
                    set_range: (0.0, 100.0),
                    #[watch]
                    #[block_signal(vol_handler)]
                    set_value: self.volume,
                    connect_value_changed[sender] => move |scale| {
                        sender.input(AppVolumeItemMsg::SliderMoved(scale.value()));
                    } @vol_handler,
                },
                gtk::Label {
                    add_css_class: "audio-app-value",
                    set_width_chars: 4,
                    set_xalign: 1.0,
                    #[watch]
                    set_label: &helpers::format_volume(self.volume),
                },
            },
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        Self {
            name: init.name,
            icon: init.icon,
            volume: init.volume,
            muted: init.muted,
            stream_index: init.stream_index,
            dragging: false,
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        _root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();

        let slider_ref = widgets.volume_slider.clone();
        let drag_sender = sender.clone();
        let controller = gtk::EventControllerLegacy::new();
        controller.connect_event(move |_, event| {
            match event.event_type() {
                gdk::EventType::ButtonPress | gdk::EventType::TouchBegin => {
                    drag_sender.input(AppVolumeItemMsg::DragStarted);
                }
                gdk::EventType::ButtonRelease
                | gdk::EventType::TouchEnd
                | gdk::EventType::TouchCancel => {
                    drag_sender.input(AppVolumeItemMsg::SliderMoved(slider_ref.value()));
                    drag_sender.input(AppVolumeItemMsg::DragEnded);
                }
                _ => {}
            }
            glib::Propagation::Proceed
        });
        widgets.volume_slider.add_controller(controller);

        widgets
    }

    fn update(&mut self, msg: Self::Input, sender: FactorySender<Self>) {
        match msg {
            AppVolumeItemMsg::SetBackendState { volume, muted } => {
                if !self.dragging {
                    self.volume = volume;
                    self.muted = muted;
                }
            }
            AppVolumeItemMsg::SliderMoved(volume) => {
                self.volume = volume;
                let _ = sender.output(AppVolumeItemOutput::VolumeChanged(
                    self.stream_index,
                    volume,
                ));
            }
            AppVolumeItemMsg::DragStarted => {
                self.dragging = true;
            }
            AppVolumeItemMsg::DragEnded => {
                self.dragging = false;
            }
            AppVolumeItemMsg::ToggleMute => {
                let _ = sender.output(AppVolumeItemOutput::ToggleMute(self.stream_index));
            }
        }
    }
}
