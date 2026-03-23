#![allow(missing_docs)]

use gtk::prelude::*;
use relm4::{WidgetTemplate, gtk, gtk::pango};

const MAX_WIDTH_CHARS: i32 = 40;
const MAX_BODY_LINES: i32 = 3;

/// Header, title, and body layout for a notification popup.
#[relm4::widget_template(pub(super))]
impl WidgetTemplate for NotificationContentTemplate {
    view! {
        gtk::Box {
            set_widget_name: "notification-text-column",
            set_css_classes: &["notification-popup-text"],
            set_orientation: gtk::Orientation::Vertical,
            set_hexpand: true,

            gtk::Box {
                set_widget_name: "notification-meta-header",
                set_css_classes: &["notification-popup-header"],

                #[name = "app_label"]
                gtk::Label {
                    add_css_class: "notification-popup-app",
                    set_halign: gtk::Align::Start,
                    set_hexpand: true,
                    set_ellipsize: pango::EllipsizeMode::End,
                },

                #[name = "time_label"]
                gtk::Label {
                    add_css_class: "notification-popup-time",
                    set_halign: gtk::Align::End,
                },
            },

            #[name = "title"]
            gtk::Label {
                add_css_class: "notification-popup-title",
                set_halign: gtk::Align::Start,
                set_ellipsize: pango::EllipsizeMode::End,
                set_max_width_chars: MAX_WIDTH_CHARS,
            },

            #[name = "body"]
            gtk::Label {
                add_css_class: "notification-popup-body",
                set_halign: gtk::Align::Start,
                set_use_markup: true,
                set_wrap: true,
                set_wrap_mode: pango::WrapMode::WordChar,
                set_max_width_chars: MAX_WIDTH_CHARS,
                set_lines: MAX_BODY_LINES,
                set_ellipsize: pango::EllipsizeMode::End,
            },
        }
    }
}
