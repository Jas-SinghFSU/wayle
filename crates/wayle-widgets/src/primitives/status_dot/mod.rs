//! Status dot widget templates for inline status indicators.
#![allow(missing_docs)]

use gtk4::prelude::WidgetExt;
use relm4::gtk;
use relm4::WidgetTemplate;

/// Default status dot with subtle foreground color.
#[relm4::widget_template(pub)]
impl WidgetTemplate for StatusDot {
    view! {
        gtk::Box {
            set_css_classes: &["status-dot"],
            set_valign: gtk::Align::Center,
        }
    }
}

/// Success status dot with green color.
#[relm4::widget_template(pub)]
impl WidgetTemplate for SuccessDot {
    view! {
        gtk::Box {
            set_css_classes: &["status-dot", "success"],
            set_valign: gtk::Align::Center,
        }
    }
}

/// Warning status dot with yellow color.
#[relm4::widget_template(pub)]
impl WidgetTemplate for WarningDot {
    view! {
        gtk::Box {
            set_css_classes: &["status-dot", "warning"],
            set_valign: gtk::Align::Center,
        }
    }
}

/// Error status dot with red color.
#[relm4::widget_template(pub)]
impl WidgetTemplate for ErrorDot {
    view! {
        gtk::Box {
            set_css_classes: &["status-dot", "error"],
            set_valign: gtk::Align::Center,
        }
    }
}

/// Info status dot with accent color.
#[relm4::widget_template(pub)]
impl WidgetTemplate for InfoDot {
    view! {
        gtk::Box {
            set_css_classes: &["status-dot", "info"],
            set_valign: gtk::Align::Center,
        }
    }
}
