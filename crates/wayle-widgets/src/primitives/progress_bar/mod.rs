//! Progress bar widget template.
#![allow(missing_docs)]

use gtk4::prelude::WidgetExt;
use relm4::{WidgetTemplate, gtk};

/// Linear progress indicator for determinate progress.
#[relm4::widget_template(pub)]
impl WidgetTemplate for ProgressBar {
    view! {
        gtk::ProgressBar {
            set_css_classes: &["progress-bar"],
        }
    }
}
