#![allow(missing_docs)]

use gtk4::prelude::{OrientableExt, WidgetExt};
use relm4::{RelmWidgetExt, WidgetTemplate, gtk};

#[relm4::widget_template(pub)]
impl WidgetTemplate for EmptyState {
    view! {
        gtk::Box {
            set_css_classes: &["empty-state"],
            set_expand: true,
            set_align: gtk::Align::Center,
            set_orientation: gtk::Orientation::Vertical,

            #[name = "icon"]
            gtk::Image {
                add_css_class: "icon",
                set_icon_name: Some("tb-alert-triangle-symbolic"),
            },

            #[name = "title"]
            gtk::Label {
                add_css_class: "title",
            },

            #[name = "description"]
            gtk::Label {
                add_css_class: "description",
            }
        }
    }
}
