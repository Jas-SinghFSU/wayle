#![allow(missing_docs)]

use gtk4::prelude::WidgetExt;
use relm4::{WidgetTemplate, gtk};

#[relm4::widget_template(pub)]
impl WidgetTemplate for Spinner {
    view! {
        gtk::Spinner {
            set_css_classes: &["spinner"],
            set_spinning: true,
        }
    }
}
