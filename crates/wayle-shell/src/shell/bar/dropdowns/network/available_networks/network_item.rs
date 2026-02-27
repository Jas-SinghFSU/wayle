use gtk::{pango, prelude::*};
use relm4::{gtk, prelude::*};

use super::methods;
use crate::{
    i18n::t,
    shell::bar::dropdowns::network::helpers::{self, AccessPointSnapshot},
};

pub(super) struct NetworkItemInit {
    pub snapshot: AccessPointSnapshot,
}

pub(super) struct NetworkItem {
    ssid: String,
    icon: &'static str,
    is_secured: bool,
    security_label: String,
}

#[derive(Debug)]
pub(super) enum NetworkItemOutput {
    Selected(DynamicIndex),
}

#[relm4::factory(pub(super))]
impl FactoryComponent for NetworkItem {
    type Init = NetworkItemInit;
    type Input = ();
    type Output = NetworkItemOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            add_css_class: "network-item",
            set_cursor_from_name: Some("pointer"),

            gtk::Image {
                add_css_class: "network-item-signal",
                #[watch]
                set_icon_name: Some(self.icon),
                set_valign: gtk::Align::Center,
            },

            gtk::Box {
                add_css_class: "network-item-info",
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,

                gtk::Label {
                    add_css_class: "network-item-name",
                    set_halign: gtk::Align::Start,
                    set_ellipsize: pango::EllipsizeMode::End,
                    #[watch]
                    set_label: &self.ssid,
                },

                gtk::Label {
                    add_css_class: "network-item-security",
                    set_halign: gtk::Align::Start,
                    #[watch]
                    set_label: &self.security_label,
                },
            },

            gtk::Image {
                add_css_class: "network-item-lock",
                set_icon_name: Some("ld-lock-symbolic"),
                set_valign: gtk::Align::Center,
                #[watch]
                set_visible: self.is_secured,
            },
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        let snapshot = init.snapshot;
        let is_secured = helpers::requires_password(snapshot.security);
        let base_label = methods::translate_security_type(snapshot.security);
        let security_label = if snapshot.known && is_secured {
            t!("dropdown-network-security-saved", security = base_label)
        } else {
            base_label
        };
        Self {
            icon: helpers::signal_strength_icon(snapshot.strength),
            is_secured,
            security_label,
            ssid: snapshot.ssid,
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
        click.connect_released(move |gesture, _, _, _| {
            gesture.set_state(gtk::EventSequenceState::Claimed);
            let _ = sender.output(NetworkItemOutput::Selected(idx.clone()));
        });
        root.add_controller(click);

        let widgets = view_output!();
        widgets
    }
}
