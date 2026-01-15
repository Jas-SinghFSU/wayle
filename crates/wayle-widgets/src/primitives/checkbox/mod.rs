//! Checkbox widget template.
#![allow(missing_docs)]

use gtk4::prelude::WidgetExt;
use relm4::{WidgetTemplate, gtk};

/// CSS class constants for Checkbox variants.
pub struct CheckboxClass;

impl CheckboxClass {
    const BASE: &'static str = "checkbox";
}

/// Checkbox for multi-select options.
///
/// Based on `gtk::CheckButton` without grouping. For single-select behavior,
/// multiple CheckButtons can be linked via `set_group()`.
#[relm4::widget_template(pub)]
impl WidgetTemplate for Checkbox {
    view! {
        gtk::CheckButton {
            set_css_classes: &[CheckboxClass::BASE],
            set_cursor_from_name: Some("pointer"),
        }
    }
}
