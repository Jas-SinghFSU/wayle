use gtk::{pango, prelude::*};
use relm4::{gtk, prelude::*};
use wayle_widgets::prelude::*;

use super::methods;
use crate::{
    i18n::t,
    shell::bar::dropdowns::network::helpers::{self, AccessPointSnapshot},
};

const HOVER_TRANSITION_MS: u32 = 150;

pub(super) struct NetworkItemInit {
    pub snapshot: AccessPointSnapshot,
}

pub(super) struct NetworkItem {
    ssid: String,
    icon: &'static str,
    security_label: String,

    is_secured: bool,
    known: bool,
    hovered: bool,
}

#[derive(Debug)]
pub(super) enum NetworkItemInput {
    Hovered(bool),
    ForgetClicked,
}

#[derive(Debug)]
pub(super) enum NetworkItemOutput {
    Selected(DynamicIndex),
    ForgetRequested(String),
}

#[relm4::factory(pub(super))]
impl FactoryComponent for NetworkItem {
    type Init = NetworkItemInit;
    type Input = NetworkItemInput;
    type Output = NetworkItemOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            add_css_class: "network-item",
            set_cursor_from_name: Some("pointer"),

            #[name = "signal_icon"]
            gtk::Image {
                add_css_class: "network-item-signal",
                #[watch]
                set_icon_name: Some(self.icon),
                set_valign: gtk::Align::Center,
            },

            #[name = "info_column"]
            gtk::Box {
                add_css_class: "network-item-info",
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,

                #[name = "ssid_label"]
                gtk::Label {
                    add_css_class: "network-item-name",
                    set_halign: gtk::Align::Start,
                    set_ellipsize: pango::EllipsizeMode::End,
                    #[watch]
                    set_label: &self.ssid,
                },

                #[name = "security_label"]
                gtk::Label {
                    add_css_class: "network-item-security",
                    set_halign: gtk::Align::Start,
                    #[watch]
                    set_label: &self.security_label,
                },
            },

            #[name = "trailing_stack"]
            gtk::Stack {
                add_css_class: "network-item-trailing",
                set_transition_type: gtk::StackTransitionType::Crossfade,
                set_transition_duration: HOVER_TRANSITION_MS,
                set_valign: gtk::Align::Center,
                set_hexpand: false,
                #[watch]
                set_visible: self.is_secured || self.known,

                add_named[Some("lock")] = &gtk::Box {
                    set_halign: gtk::Align::End,
                    set_valign: gtk::Align::Center,

                    #[name = "lock_icon"]
                    gtk::Image {
                        add_css_class: "network-item-lock",
                        set_icon_name: Some("ld-lock-symbolic"),
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_visible: self.is_secured,
                    },
                },

                add_named[Some("actions")] = &gtk::Box {
                    add_css_class: "network-item-actions",
                    set_valign: gtk::Align::Center,

                    #[template]
                    GhostButton {
                        add_css_class: "network-item-forget",
                        #[template_child]
                        label {
                            set_label: &t!("dropdown-network-forget"),
                        },
                        connect_clicked => NetworkItemInput::ForgetClicked,
                    },
                },

                #[watch]
                set_visible_child_name:
                    if self.hovered && self.known {
                        "actions"
                    } else {
                        "lock"
                    },
            },
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        let snapshot = init.snapshot;
        let is_secured = helpers::requires_password(snapshot.security);
        let known = snapshot.known;
        let base_label = methods::translate_security_type(snapshot.security);

        let security_label = if known && is_secured {
            t!("dropdown-network-security-saved", security = base_label)
        } else {
            base_label
        };

        Self {
            icon: helpers::signal_strength_icon(snapshot.strength),
            is_secured,
            known,
            hovered: false,
            security_label,
            ssid: snapshot.ssid,
        }
    }

    fn update(&mut self, msg: NetworkItemInput, sender: FactorySender<Self>) {
        match msg {
            NetworkItemInput::Hovered(hovered) => {
                self.hovered = hovered;
            }

            NetworkItemInput::ForgetClicked => {
                let _ = sender.output(NetworkItemOutput::ForgetRequested(self.ssid.clone()));
            }
        }
    }

    fn init_widgets(
        &mut self,
        index: &Self::Index,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let click = gtk::GestureClick::new();
        let idx = index.clone();
        let click_sender = sender.output_sender().clone();

        click.connect_released(move |gesture, _, _, _| {
            gesture.set_state(gtk::EventSequenceState::Claimed);
            click_sender.emit(NetworkItemOutput::Selected(idx.clone()));
        });

        root.add_controller(click);

        if self.known {
            let hover = gtk::EventControllerMotion::new();
            let hover_sender = sender.input_sender().clone();

            hover.connect_enter(move |_, _, _| {
                hover_sender.emit(NetworkItemInput::Hovered(true));
            });

            let leave_sender = sender.input_sender().clone();

            hover.connect_leave(move |_| {
                leave_sender.emit(NetworkItemInput::Hovered(false));
            });

            root.add_controller(hover);
        }

        let widgets = view_output!();
        widgets
    }
}
