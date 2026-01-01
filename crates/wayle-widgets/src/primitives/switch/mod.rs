//! Switch widget template.
#![allow(missing_docs)]

use gtk4::prelude::WidgetExt;
use relm4::{WidgetTemplate, gtk};

/// Toggle switch for binary on/off states.
#[relm4::widget_template(pub)]
impl WidgetTemplate for Switch {
    view! {
        gtk::Switch {
            set_cursor_from_name: Some("pointer"),
        }
    }
}
