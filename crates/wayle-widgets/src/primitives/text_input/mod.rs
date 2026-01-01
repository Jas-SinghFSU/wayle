//! Text input widget template.
#![allow(missing_docs)]

use gtk4::prelude::WidgetExt;
use relm4::{gtk, WidgetTemplate};

/// CSS class constants for TextInput states.
///
/// ```ignore
/// #[template]
/// TextInput {
///     add_css_class: TextInputClass::ERROR,
/// }
/// ```
pub struct TextInputClass;

impl TextInputClass {
    const BASE: &'static str = "text-input";

    /// Error state styling.
    pub const ERROR: &'static str = "error";
    /// Warning state styling.
    pub const WARNING: &'static str = "warning";
}

/// Single-line text entry field.
#[relm4::widget_template(pub)]
impl WidgetTemplate for TextInput {
    view! {
        gtk::Entry {
            set_css_classes: &[TextInputClass::BASE],
        }
    }
}
