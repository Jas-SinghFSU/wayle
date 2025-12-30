//! Alert widget templates for status messages and notifications.
#![allow(missing_docs)]

use gtk4::prelude::{OrientableExt, WidgetExt};
use relm4::{WidgetTemplate, gtk};

/// Default alert with neutral styling.
#[relm4::widget_template(pub)]
impl WidgetTemplate for Alert {
    view! {
        gtk::Box {
            set_css_classes: &["alert"],

            #[name = "icon"]
            gtk::Image {
                set_css_classes: &["alert-icon"],
                set_valign: gtk::Align::Start,
                set_icon_name: Some("tb-info-circle-symbolic"),
            },

            gtk::Box {
                set_css_classes: &["alert-content"],
                set_orientation: gtk::Orientation::Vertical,

                #[name = "title"]
                gtk::Label {
                    set_css_classes: &["alert-title"],
                    set_halign: gtk::Align::Start,
                },

                #[name = "description"]
                gtk::Label {
                    set_css_classes: &["alert-description"],
                    set_halign: gtk::Align::Start,
                    set_wrap: true,
                    set_visible: false,
                },
            },
        }
    }
}

/// Success alert for positive confirmations.
#[relm4::widget_template(pub)]
impl WidgetTemplate for SuccessAlert {
    view! {
        gtk::Box {
            set_css_classes: &["alert", "success"],

            #[name = "icon"]
            gtk::Image {
                set_css_classes: &["alert-icon"],
                set_valign: gtk::Align::Start,
                set_icon_name: Some("tb-check-symbolic"),
            },

            gtk::Box {
                set_css_classes: &["alert-content"],
                set_orientation: gtk::Orientation::Vertical,

                #[name = "title"]
                gtk::Label {
                    set_css_classes: &["alert-title"],
                    set_halign: gtk::Align::Start,
                },

                #[name = "description"]
                gtk::Label {
                    set_css_classes: &["alert-description"],
                    set_halign: gtk::Align::Start,
                    set_wrap: true,
                    set_visible: false,
                },
            },
        }
    }
}

/// Warning alert for caution messages.
#[relm4::widget_template(pub)]
impl WidgetTemplate for WarningAlert {
    view! {
        gtk::Box {
            set_css_classes: &["alert", "warning"],

            #[name = "icon"]
            gtk::Image {
                set_css_classes: &["alert-icon"],
                set_valign: gtk::Align::Start,
                set_icon_name: Some("tb-alert-triangle-symbolic"),
            },

            gtk::Box {
                set_css_classes: &["alert-content"],
                set_orientation: gtk::Orientation::Vertical,

                #[name = "title"]
                gtk::Label {
                    set_css_classes: &["alert-title"],
                    set_halign: gtk::Align::Start,
                },

                #[name = "description"]
                gtk::Label {
                    set_css_classes: &["alert-description"],
                    set_halign: gtk::Align::Start,
                    set_wrap: true,
                    set_visible: false,
                },
            },
        }
    }
}

/// Error alert for failure messages.
#[relm4::widget_template(pub)]
impl WidgetTemplate for ErrorAlert {
    view! {
        gtk::Box {
            set_css_classes: &["alert", "error"],

            #[name = "icon"]
            gtk::Image {
                set_css_classes: &["alert-icon"],
                set_valign: gtk::Align::Start,
                set_icon_name: Some("tb-xbox-x-symbolic"),
            },

            gtk::Box {
                set_css_classes: &["alert-content"],
                set_orientation: gtk::Orientation::Vertical,

                #[name = "title"]
                gtk::Label {
                    set_css_classes: &["alert-title"],
                    set_halign: gtk::Align::Start,
                },

                #[name = "description"]
                gtk::Label {
                    set_css_classes: &["alert-description"],
                    set_halign: gtk::Align::Start,
                    set_wrap: true,
                    set_visible: false,
                },
            },
        }
    }
}

/// Info alert for informational messages.
#[relm4::widget_template(pub)]
impl WidgetTemplate for InfoAlert {
    view! {
        gtk::Box {
            set_css_classes: &["alert", "info"],

            #[name = "icon"]
            gtk::Image {
                set_css_classes: &["alert-icon"],
                set_valign: gtk::Align::Start,
                set_icon_name: Some("tb-info-circle-symbolic"),
            },

            gtk::Box {
                set_css_classes: &["alert-content"],
                set_orientation: gtk::Orientation::Vertical,

                #[name = "title"]
                gtk::Label {
                    set_css_classes: &["alert-title"],
                    set_halign: gtk::Align::Start,
                },

                #[name = "description"]
                gtk::Label {
                    set_css_classes: &["alert-description"],
                    set_halign: gtk::Align::Start,
                    set_wrap: true,
                    set_visible: false,
                },
            },
        }
    }
}
