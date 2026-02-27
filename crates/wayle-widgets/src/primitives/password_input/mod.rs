//! Password input widget template.
#![allow(missing_docs)]

use relm4::{WidgetTemplate, gtk};

/// Password entry field with built-in visibility toggle.
#[relm4::widget_template(pub)]
impl WidgetTemplate for PasswordInput {
    view! {
        gtk::PasswordEntry {
            set_show_peek_icon: true,
        }
    }
}
