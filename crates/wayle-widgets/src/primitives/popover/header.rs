//! Popover header template.
#![allow(missing_docs)]

use gtk4::prelude::WidgetExt;
use relm4::{WidgetTemplate, gtk};

/// Header section for popover menus.
#[relm4::widget_template(pub)]
impl WidgetTemplate for PopoverHeader {
    view! {
        gtk::Box {
            set_css_classes: &["popover-header"],

            #[name = "label"]
            gtk::Label {
                set_css_classes: &["popover-header-label"],
            }
        }
    }
}
