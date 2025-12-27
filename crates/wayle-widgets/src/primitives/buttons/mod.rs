use gtk4::prelude::BoxExt;
use gtk4::prelude::WidgetExt;
use relm4::gtk;
use relm4::WidgetTemplate;

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

#[relm4::widget_template(pub)]
impl WidgetTemplate for GhostIconButton {
    view! {
        gtk::Button {
            set_css_classes: &["btn", "btn-ghost-icon"],
            set_cursor_from_name: Some("pointer"),
        }
    }
}

#[relm4::widget_template(pub)]
impl WidgetTemplate for IconButton {
    view! {
        gtk::Button {
            set_css_classes: &["btn", "btn-icon"],
            set_cursor_from_name: Some("pointer"),
        }
    }
}

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
