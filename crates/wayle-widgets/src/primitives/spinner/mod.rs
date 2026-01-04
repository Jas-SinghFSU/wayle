//! Spinner widget template.
#![allow(missing_docs)]

use relm4::{WidgetTemplate, gtk};

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
            set_spinning: true,
        }
    }
}
