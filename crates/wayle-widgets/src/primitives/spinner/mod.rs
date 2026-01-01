//! Spinner widget template.
#![allow(missing_docs)]

use gtk4::prelude::WidgetExt;
use relm4::{gtk, WidgetTemplate};

/// CSS class constants for Spinner sizes.
///
/// ```ignore
/// #[template]
/// Spinner {
///     add_css_class: SpinnerClass::SM,
/// }
/// ```
pub struct SpinnerClass;

impl SpinnerClass {
    const BASE: &'static str = "spinner";

    /// Small size variant.
    pub const SM: &'static str = "sm";
    /// Large size variant.
    pub const LG: &'static str = "lg";
}

/// Loading indicator with animated rotation.
#[relm4::widget_template(pub)]
impl WidgetTemplate for Spinner {
    view! {
        gtk::Spinner {
            set_css_classes: &[SpinnerClass::BASE],
            set_spinning: true,
        }
    }
}
