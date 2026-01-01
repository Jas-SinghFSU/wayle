//! Progress bar widget template.
#![allow(missing_docs)]

use gtk4::prelude::WidgetExt;
use relm4::{WidgetTemplate, gtk};

/// CSS class constants for ProgressBar color variants.
///
/// ```ignore
/// #[template]
/// ProgressBar {
///     add_css_class: ProgressBarClass::SUCCESS,
/// }
/// ```
pub struct ProgressBarClass;

impl ProgressBarClass {
    const BASE: &'static str = "progress-bar";

    /// Success/positive color.
    pub const SUCCESS: &'static str = "success";
    /// Warning/caution color.
    pub const WARNING: &'static str = "warning";
    /// Error/negative color.
    pub const ERROR: &'static str = "error";
    /// Compact size variant.
    pub const SMALL: &'static str = "sm";
    /// Large size variant.
    pub const LARGE: &'static str = "lg";
}

/// Linear progress indicator for determinate progress.
#[relm4::widget_template(pub)]
impl WidgetTemplate for ProgressBar {
    view! {
        gtk::ProgressBar {
            set_css_classes: &[ProgressBarClass::BASE],
        }
    }
}
