//! Text input widget template.
#![allow(missing_docs)]

use gtk4::prelude::WidgetExt;
use relm4::WidgetTemplate;
use relm4::gtk;

/// Single-line text entry field.
#[relm4::widget_template(pub)]
impl WidgetTemplate for TextInput {
    view! {
        gtk::Entry {
            set_css_classes: &["text-input"],
        }
    }
}
