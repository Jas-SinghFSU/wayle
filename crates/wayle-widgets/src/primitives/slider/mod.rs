//! Slider widget template.
#![allow(missing_docs)]

use gtk4::prelude::*;
use relm4::{WidgetTemplate, gtk};

/// Horizontal range slider with origin highlight.
#[relm4::widget_template(pub)]
impl WidgetTemplate for Slider {
    view! {
        gtk::Scale {
            set_css_classes: &["slider"],
            set_draw_value: false,
            set_cursor_from_name: Some("pointer"),
            set_has_origin: true,
        }
    }
}
