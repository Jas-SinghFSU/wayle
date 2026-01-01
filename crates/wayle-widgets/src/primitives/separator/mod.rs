//! Separator widget
#![allow(missing_docs)]

use gtk4::prelude::{OrientableExt, WidgetExt};
use relm4::{WidgetTemplate, gtk};

#[relm4::widget_template(pub)]
impl WidgetTemplate for HorizontalSeparator {
    view! {
        gtk::Separator {
            set_hexpand: true,
            set_valign: gtk::Align::Center,
        }
    }
}

#[relm4::widget_template(pub)]
impl WidgetTemplate for VerticalSeparator {
    view! {
        gtk::Separator {
            set_orientation: gtk::Orientation::Vertical,
            set_vexpand: true,
            set_halign: gtk::Align::Center,
        }
    }
}
