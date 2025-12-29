//! Button widget templates.
#![allow(missing_docs)]

use gtk4::prelude::{BoxExt, WidgetExt};
use relm4::{WidgetTemplate, gtk};

/// Primary action button with accent background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for PrimaryButton {
    view! {
        gtk::Button {
            set_css_classes: &["btn", "btn-primary"],
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
                set_spacing: 8,
                #[name = "icon"]
                gtk::Image {
                    set_visible: false,
                },
                #[name = "label"]
                gtk::Label {},
            },
        }
    }
}

/// Secondary action button with subtle background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for SecondaryButton {
    view! {
        gtk::Button {
            set_css_classes: &["btn", "btn-secondary"],
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
                set_spacing: 8,
                #[name = "icon"]
                gtk::Image {
                    set_visible: false,
                },
                #[name = "label"]
                gtk::Label {},
            },
        }
    }
}

/// Destructive action button with error-colored background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for DangerButton {
    view! {
        gtk::Button {
            set_css_classes: &["btn", "btn-danger"],
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
                set_spacing: 8,
                #[name = "icon"]
                gtk::Image {
                    set_visible: false,
                },
                #[name = "label"]
                gtk::Label {},
            },
        }
    }
}

/// Transparent button with text, no background until hover.
#[relm4::widget_template(pub)]
impl WidgetTemplate for GhostButton {
    view! {
        gtk::Button {
            set_css_classes: &["btn", "btn-ghost"],
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
                set_spacing: 8,
                #[name = "icon"]
                gtk::Image {
                    set_visible: false,
                },
                #[name = "label"]
                gtk::Label {},
            },
        }
    }
}

/// Icon-only ghost button.
#[relm4::widget_template(pub)]
impl WidgetTemplate for GhostIconButton {
    view! {
        gtk::Button {
            set_css_classes: &["btn", "btn-ghost-icon"],
            set_cursor_from_name: Some("pointer"),
        }
    }
}

/// Icon-only button with background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for IconButton {
    view! {
        gtk::Button {
            set_css_classes: &["btn", "btn-icon"],
            set_cursor_from_name: Some("pointer"),
        }
    }
}

/// Text-only link-styled button.
#[relm4::widget_template(pub)]
impl WidgetTemplate for LinkButton {
    view! {
        gtk::Button {
            set_css_classes: &["btn-link"],
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
                set_spacing: 8,
                #[name = "icon"]
                gtk::Image {
                    set_visible: false,
                },
                #[name = "label"]
                gtk::Label {},
            },
        }
    }
}

/// Muted link button for secondary actions.
#[relm4::widget_template(pub)]
impl WidgetTemplate for MutedLinkButton {
    view! {
        gtk::Button {
            set_css_classes: &["btn-link", "muted"],
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
                set_spacing: 8,
                #[name = "icon"]
                gtk::Image {
                    set_visible: false,
                },
                #[name = "label"]
                gtk::Label {},
            },
        }
    }
}

/// Danger-colored link button for destructive actions.
#[relm4::widget_template(pub)]
impl WidgetTemplate for DangerLinkButton {
    view! {
        gtk::Button {
            set_css_classes: &["btn-link", "danger"],
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
                set_spacing: 8,
                #[name = "icon"]
                gtk::Image {
                    set_visible: false,
                },
                #[name = "label"]
                gtk::Label {},
            },
        }
    }
}

/// Button that opens a popover menu.
#[relm4::widget_template(pub)]
impl WidgetTemplate for MenuButton {
    view! {
        gtk::MenuButton {
            set_css_classes: &["btn-menu"],
            set_cursor_from_name: Some("pointer"),
            set_always_show_arrow: true,

            #[wrap(Some)]
            #[name = "label"]
            set_child = &gtk::Label {
                set_xalign: 0.0,
            },
        }
    }
}
