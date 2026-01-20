//! Button widget templates.
#![allow(missing_docs)]

use gtk4::prelude::WidgetExt;
use relm4::{WidgetTemplate, gtk};

/// CSS class constants for link button modifiers.
pub struct LinkButtonClass;

impl LinkButtonClass {
    /// Muted/subdued text color for secondary links.
    pub const MUTED: &'static str = "muted";
    /// Danger/error color for destructive links.
    pub const DANGER: &'static str = "danger";
}

/// Primary action button with accent background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for PrimaryButton {
    view! {
        gtk::Button {
            add_css_class: "primary",
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
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
            add_css_class: "secondary",
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
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
            add_css_class: "danger",
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
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
            add_css_class: "ghost",
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
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
            add_css_class: "ghost-icon",
            set_cursor_from_name: Some("pointer"),
        }
    }
}

/// Icon-only button with background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for IconButton {
    view! {
        gtk::Button {
            add_css_class: "icon",
            set_cursor_from_name: Some("pointer"),
        }
    }
}

/// Text-only link-styled button.
#[relm4::widget_template(pub)]
impl WidgetTemplate for LinkButton {
    view! {
        gtk::Button {
            add_css_class: "link",
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
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
            set_css_classes: &["link", LinkButtonClass::MUTED],
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
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
            set_css_classes: &["link", LinkButtonClass::DANGER],
            set_cursor_from_name: Some("pointer"),
            gtk::Box {
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
            add_css_class: "menu",
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
