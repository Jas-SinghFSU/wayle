//! Dropdown container widget templates.
#![allow(missing_docs)]

use gtk4::prelude::{OrientableExt, WidgetExt};
use relm4::{WidgetTemplate, gtk};

/// Main dropdown container.
#[relm4::widget_template(pub)]
impl WidgetTemplate for Dropdown {
    view! {
        gtk::Box {
            set_css_classes: &["dropdown"],
            set_orientation: gtk::Orientation::Vertical,
            set_hexpand: false,
        }
    }
}

/// Header with icon, label, and actions container.
#[relm4::widget_template(pub)]
impl WidgetTemplate for DropdownHeader {
    view! {
        gtk::Box {
            set_css_classes: &["dropdown-header"],

            gtk::Box {
                set_css_classes: &["dropdown-title"],
                set_hexpand: true,

                #[name = "icon"]
                gtk::Image {
                    set_visible: false,
                },

                #[name = "label"]
                gtk::Label {},
            },

            #[name = "actions"]
            gtk::Box {
                set_css_classes: &["dropdown-actions"],
            },
        }
    }
}

/// Footer container.
#[relm4::widget_template(pub)]
impl WidgetTemplate for DropdownFooter {
    view! {
        gtk::Box {
            set_css_classes: &["dropdown-footer"],
            set_halign: gtk::Align::Fill,
            set_hexpand: true,
        }
    }
}

/// Content area container.
#[relm4::widget_template(pub)]
impl WidgetTemplate for DropdownContent {
    view! {
        gtk::Box {
            set_css_classes: &["dropdown-content"],
            set_orientation: gtk::Orientation::Vertical,
        }
    }
}
