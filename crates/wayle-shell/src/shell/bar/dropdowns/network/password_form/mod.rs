mod messages;
mod methods;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_widgets::prelude::*;

pub(crate) use self::messages::*;
use crate::i18n::t;

const ICON_EYE: &str = "ld-eye-symbolic";
const ICON_EYE_OFF: &str = "ld-eye-off-symbolic";

pub(crate) struct PasswordForm {
    ssid: String,
    security_label: String,
    signal_icon: &'static str,
    visible: bool,
    error_message: Option<String>,
    password_entry: gtk::Entry,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for PasswordForm {
    type Init = ();
    type Input = PasswordFormInput;
    type Output = PasswordFormOutput;

    view! {
        #[root]
        gtk::Box {
            add_css_class: "network-password-card",
            set_orientation: gtk::Orientation::Vertical,
            #[watch]
            set_visible: model.visible,

            gtk::Box {
                add_css_class: "network-password-header",

                gtk::Box {
                    add_css_class: "network-connection-icon",
                    add_css_class: "wifi",
                    set_hexpand: false,
                    gtk::Image {
                        #[watch]
                        set_icon_name: Some(model.signal_icon),
                        set_hexpand: true,
                    },
                },

                gtk::Box {
                    add_css_class: "network-password-info",
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,

                    gtk::Label {
                        add_css_class: "network-password-name",
                        set_halign: gtk::Align::Start,
                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                        #[watch]
                        set_label: &model.ssid,
                    },

                    gtk::Label {
                        add_css_class: "network-password-security",
                        set_halign: gtk::Align::Start,
                        #[watch]
                        set_label: &model.security_label,
                    },
                },

                #[template]
                GhostIconButton {
                    add_css_class: "network-password-close",
                    set_icon_name: "ld-x-symbolic",
                    set_valign: gtk::Align::Start,
                    connect_clicked => PasswordFormInput::CancelClicked,
                },
            },

            model.password_entry.clone() -> gtk::Entry {
                add_css_class: "network-password-input",
                set_visibility: false,
                set_input_purpose: gtk::InputPurpose::Password,
                set_focusable: true,
                set_can_target: true,
                set_placeholder_text: Some(&t!("dropdown-network-password-placeholder")),
                connect_activate => PasswordFormInput::ConnectClicked,
            },

            gtk::Label {
                add_css_class: "network-password-error",
                set_halign: gtk::Align::Start,
                #[watch]
                set_visible: model.error_message.is_some(),
                #[watch]
                set_label: model.error_message.as_deref().unwrap_or(""),
            },

            gtk::Box {
                add_css_class: "network-password-actions",
                set_halign: gtk::Align::End,

                #[template]
                GhostButton {
                    add_css_class: "network-password-cancel",
                    connect_clicked => PasswordFormInput::CancelClicked,
                    #[template_child]
                    label {
                        set_label: &t!("dropdown-network-cancel"),
                    },
                },

                #[template]
                PrimaryButton {
                    add_css_class: "network-password-connect",
                    connect_clicked => PasswordFormInput::ConnectClicked,
                    #[template_child]
                    label {
                        set_label: &t!("dropdown-network-connect"),
                    },
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let password_entry = Self::build_password_entry();

        let model = Self {
            ssid: String::new(),
            security_label: String::new(),
            signal_icon: "cm-wireless-signal-excellent-symbolic",
            visible: false,
            error_message: None,
            password_entry,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            PasswordFormInput::Show {
                ssid,
                security_label,
                signal_icon,
                error_message,
            } => {
                self.ssid = ssid;
                self.security_label = security_label;
                self.signal_icon = signal_icon;
                self.error_message = error_message;
                self.reset_entry();
                self.visible = true;
                self.password_entry.grab_focus();
            }
            PasswordFormInput::ConnectClicked => {
                let password = self.password_entry.text().to_string();
                let _ = sender.output(PasswordFormOutput::Connect { password });
                self.visible = false;
            }
            PasswordFormInput::CancelClicked => {
                let _ = sender.output(PasswordFormOutput::Cancel);
                self.visible = false;
            }
        }
    }
}
