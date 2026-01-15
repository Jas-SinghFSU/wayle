//! Confirmation modal for destructive or irreversible actions.
#![allow(missing_docs)]

use gtk::prelude::*;
use relm4::prelude::*;

use crate::primitives::buttons::{DangerButton, PrimaryButton, SecondaryButton};

/// Icon style for the modal header.
#[derive(Debug, Clone, Copy, Default)]
pub enum ModalIcon {
    /// Warning icon with yellow/orange styling.
    #[default]
    Warning,
    /// Error icon with red styling.
    Error,
    /// Success icon with green styling.
    Success,
    /// Info icon with accent/blue styling.
    Info,
    /// No icon displayed.
    None,
}

impl ModalIcon {
    fn icon_name(self) -> Option<&'static str> {
        match self {
            Self::Warning => Some("tb-alert-triangle-symbolic"),
            Self::Error => Some("tb-xbox-x-symbolic"),
            Self::Success => Some("tb-check-symbolic"),
            Self::Info => Some("tb-info-circle-symbolic"),
            Self::None => None,
        }
    }

    fn css_classes(self) -> &'static [&'static str] {
        match self {
            Self::Warning => &["modal-icon", "warning"],
            Self::Error => &["modal-icon", "error"],
            Self::Success => &["modal-icon", "success"],
            Self::Info => &["modal-icon", "info"],
            Self::None => &["modal-icon"],
        }
    }
}

/// Style for the confirm button.
#[derive(Debug, Clone, Copy, Default)]
pub enum ConfirmStyle {
    /// Red destructive styling for dangerous actions.
    #[default]
    Danger,
    /// Accent/blue primary styling for non-destructive confirmations.
    Primary,
}

/// Configuration for displaying a confirmation modal.
#[derive(Debug, Clone)]
pub struct ConfirmModalConfig {
    /// Title text displayed in the modal header.
    pub title: String,
    /// Optional description providing additional context.
    pub description: Option<String>,
    /// Icon style for the modal header.
    pub icon: ModalIcon,
    /// Label for the confirm button.
    pub confirm_label: String,
    /// Style for the confirm button.
    pub confirm_style: ConfirmStyle,
    /// Label for the cancel button. Defaults to "Cancel".
    pub cancel_label: Option<String>,
}

/// Output messages from the confirmation modal.
#[derive(Debug, Clone)]
pub enum ConfirmModalOutput {
    /// User confirmed the action.
    Confirmed,
    /// User cancelled the action (cancel button, ESC, or window close).
    Cancelled,
}

/// Input messages for the confirmation modal.
#[derive(Debug)]
pub enum ConfirmModalMsg {
    /// Show the modal with the given configuration.
    Show(ConfirmModalConfig),
    /// Hide the modal (internal use).
    Hide,
    /// User clicked confirm button.
    Confirm,
    /// User clicked cancel button.
    Cancel,
}

/// Confirmation modal component.
///
/// Initialized once and controlled via `Show(config)` messages.
/// Emits `Confirmed` or `Cancelled` output based on user interaction.
pub struct ConfirmModal {
    visible: bool,
    title: String,
    description: Option<String>,
    icon: ModalIcon,
    confirm_label: String,
    confirm_style: ConfirmStyle,
    cancel_label: String,
}

impl Default for ConfirmModal {
    fn default() -> Self {
        Self {
            visible: false,
            title: String::new(),
            description: None,
            icon: ModalIcon::Warning,
            confirm_label: "Confirm".to_string(),
            confirm_style: ConfirmStyle::Danger,
            cancel_label: "Cancel".to_string(),
        }
    }
}

#[relm4::component(pub)]
impl SimpleComponent for ConfirmModal {
    type Init = ();
    type Input = ConfirmModalMsg;
    type Output = ConfirmModalOutput;

    view! {
        gtk::Window {
            #[watch]
            set_visible: model.visible,
            set_modal: true,
            set_decorated: false,
            set_resizable: false,
            add_css_class: "modal",

            connect_close_request[sender] => move |_| {
                sender.input(ConfirmModalMsg::Cancel);
                glib::Propagation::Stop
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::Box {
                    add_css_class: "modal-header",

                    gtk::Box {
                        #[watch]
                        set_css_classes: model.icon.css_classes(),
                        set_expand: false,
                        set_halign: gtk::Align::Start,
                        set_valign: gtk::Align::Start,
                        #[watch]
                        set_visible: model.icon.icon_name().is_some(),

                        gtk::Image {
                            set_expand: true,
                            set_halign: gtk::Align::Fill,
                            set_valign: gtk::Align::Fill,
                            #[watch]
                            set_icon_name: model.icon.icon_name(),
                        },
                    },

                    gtk::Box {
                        add_css_class: "modal-header-content",
                        set_orientation: gtk::Orientation::Vertical,
                        set_hexpand: true,

                        gtk::Label {
                            add_css_class: "modal-title",
                            set_halign: gtk::Align::Fill,
                            set_xalign: 0.0,
                            set_max_width_chars: 30,
                            set_wrap: true,
                            #[watch]
                            set_label: &model.title,
                        },

                        gtk::Label {
                            add_css_class: "modal-description",
                            set_halign: gtk::Align::Fill,
                            set_xalign: 0.0,
                            set_max_width_chars: 35,
                            set_wrap: true,
                            #[watch]
                            set_visible: model.description.is_some(),
                            #[watch]
                            set_label: model.description.as_deref().unwrap_or(""),
                        },
                    },
                },

                gtk::Box {
                    add_css_class: "modal-footer",
                    set_halign: gtk::Align::End,

                    #[template]
                    SecondaryButton {
                        #[template_child]
                        label {
                            #[watch]
                            set_label: &model.cancel_label,
                        },
                        connect_clicked => ConfirmModalMsg::Cancel,
                    },

                    #[template]
                    DangerButton {
                        #[watch]
                        set_visible: matches!(model.confirm_style, ConfirmStyle::Danger),
                        #[template_child]
                        label {
                            #[watch]
                            set_label: &model.confirm_label,
                        },
                        connect_clicked => ConfirmModalMsg::Confirm,
                    },

                    #[template]
                    PrimaryButton {
                        #[watch]
                        set_visible: matches!(model.confirm_style, ConfirmStyle::Primary),
                        #[template_child]
                        label {
                            #[watch]
                            set_label: &model.confirm_label,
                        },
                        connect_clicked => ConfirmModalMsg::Confirm,
                    },
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self::default();
        let widgets = view_output!();

        let key_controller = gtk::EventControllerKey::new();
        key_controller.connect_key_pressed({
            let sender = sender.clone();
            move |_, key, _, _| {
                if key == gtk::gdk::Key::Escape {
                    sender.input(ConfirmModalMsg::Cancel);
                    glib::Propagation::Stop
                } else {
                    glib::Propagation::Proceed
                }
            }
        });
        root.add_controller(key_controller);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            ConfirmModalMsg::Show(config) => {
                self.title = config.title;
                self.description = config.description;
                self.icon = config.icon;
                self.confirm_label = config.confirm_label;
                self.confirm_style = config.confirm_style;
                self.cancel_label = config.cancel_label.unwrap_or_else(|| "Cancel".to_string());
                self.visible = true;
            }
            ConfirmModalMsg::Hide => {
                self.visible = false;
            }
            ConfirmModalMsg::Confirm => {
                self.visible = false;
                let _ = sender.output(ConfirmModalOutput::Confirmed);
            }
            ConfirmModalMsg::Cancel => {
                self.visible = false;
                let _ = sender.output(ConfirmModalOutput::Cancelled);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modal_icon_returns_correct_icon_names() {
        assert_eq!(
            ModalIcon::Warning.icon_name(),
            Some("tb-alert-triangle-symbolic")
        );
        assert_eq!(ModalIcon::Error.icon_name(), Some("tb-xbox-x-symbolic"));
        assert_eq!(ModalIcon::Success.icon_name(), Some("tb-check-symbolic"));
        assert_eq!(ModalIcon::Info.icon_name(), Some("tb-info-circle-symbolic"));
        assert_eq!(ModalIcon::None.icon_name(), None);
    }

    #[test]
    fn modal_icon_css_classes_always_include_base_class() {
        for icon in [
            ModalIcon::Warning,
            ModalIcon::Error,
            ModalIcon::Success,
            ModalIcon::Info,
            ModalIcon::None,
        ] {
            assert!(
                icon.css_classes().contains(&"modal-icon"),
                "{icon:?} missing base 'modal-icon' class"
            );
        }
    }

    #[test]
    fn modal_icon_css_classes_include_variant_class() {
        assert!(ModalIcon::Warning.css_classes().contains(&"warning"));
        assert!(ModalIcon::Error.css_classes().contains(&"error"));
        assert!(ModalIcon::Success.css_classes().contains(&"success"));
        assert!(ModalIcon::Info.css_classes().contains(&"info"));
    }

    #[test]
    fn modal_icon_none_has_only_base_class() {
        let classes = ModalIcon::None.css_classes();
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0], "modal-icon");
    }

    #[test]
    fn default_modal_icon_is_warning() {
        assert!(matches!(ModalIcon::default(), ModalIcon::Warning));
    }

    #[test]
    fn default_confirm_style_is_danger() {
        assert!(matches!(ConfirmStyle::default(), ConfirmStyle::Danger));
    }
}
