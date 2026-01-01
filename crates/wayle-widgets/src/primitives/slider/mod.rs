//! Slider widgets: continuous template and stepped component.
#![allow(missing_docs)]

mod stepped;

use gtk4::prelude::*;
use relm4::{gtk, WidgetTemplate};

pub use stepped::{
    EmitMode, SteppedSlider, SteppedSliderInit, SteppedSliderMsg, SteppedSliderOutput,
};

/// Horizontal range slider with origin highlight.
#[relm4::widget_template(pub)]
impl WidgetTemplate for Slider {
    view! {
        gtk::Scale {
            set_draw_value: false,
            set_cursor_from_name: Some("pointer"),
            set_has_origin: true,
        }
    }
}
