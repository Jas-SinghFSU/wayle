//! Slider widgets: continuous template, stepped component, and debounced slider.
#![allow(missing_docs)]

pub mod debounced;
mod stepped;

pub use debounced::DebouncedSlider;
use gtk4::prelude::*;
use relm4::{WidgetTemplate, gtk};
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
