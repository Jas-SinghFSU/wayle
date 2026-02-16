use gtk::{pango, prelude::*};
use relm4::{gtk, prelude::*};

use super::messages::DeviceInfo;

pub(super) struct DeviceOptionItem {
    description: String,
    subtitle: Option<String>,
    icon: &'static str,
    is_active: bool,
}

#[relm4::factory(pub(super))]
impl FactoryComponent for DeviceOptionItem {
    type Init = DeviceInfo;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        gtk::ListBoxRow {
            add_css_class: "audio-device-option",
            set_activatable: true,
            set_cursor_from_name: Some("pointer"),
            #[watch]
            set_css_classes: if self.is_active {
                &["audio-device-option", "selected"]
            } else {
                &["audio-device-option"]
            },

            gtk::Box {
                add_css_class: "audio-device-option-content",

                gtk::CenterBox {
                    add_css_class: "audio-device-option-icon",
                    set_valign: gtk::Align::Center,
                    #[wrap(Some)]
                    set_center_widget = &gtk::Image {
                        set_icon_name: Some(self.icon),
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,
                    set_halign: gtk::Align::Start,
                    set_valign: gtk::Align::Center,

                    gtk::Label {
                        add_css_class: "audio-device-option-name",
                        set_halign: gtk::Align::Start,
                        set_ellipsize: pango::EllipsizeMode::End,
                        #[watch]
                        set_label: &self.description,
                    },

                    gtk::Label {
                        add_css_class: "audio-device-option-subtitle",
                        set_halign: gtk::Align::Start,
                        set_ellipsize: pango::EllipsizeMode::End,
                        #[watch]
                        set_visible: self.subtitle.is_some(),
                        #[watch]
                        set_label: self.subtitle.as_deref().unwrap_or_default(),
                    },
                },

                gtk::Image {
                    add_css_class: "audio-device-option-check",
                    set_icon_name: Some("tb-check-symbolic"),
                    #[watch]
                    set_visible: self.is_active,
                },
            },
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        Self {
            description: init.description,
            subtitle: init.subtitle,
            icon: init.icon,
            is_active: init.is_active,
        }
    }
}
