pub(super) mod messages;
mod methods;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_widgets::prelude::*;

use self::messages::{PairingCardInit, PairingCardMsg};
use super::messages::PairingCardOutput;
use crate::i18n::{t, td};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PairingVariant {
    None,
    DisplayPin,
    RequestPasskey,
    DisplayPasskey,
    RequestConfirmation,
    RequestAuthorization,
    RequestServiceAuthorization,
    RequestPinCode,
    Failed,
}

pub(super) struct PairingCard {
    variant: PairingVariant,
    device_name: String,
    device_type: String,
    device_icon: &'static str,
    pin_code: String,
    passkey_entered: u16,
    passkey_total: u16,
    service_uuid: String,
    legacy_pin: String,
    pin_entries: [gtk::Entry; 6],
    legacy_pin_entry: gtk::Entry,
}

#[relm4::component(pub(super))]
impl SimpleComponent for PairingCard {
    type Init = PairingCardInit;
    type Input = PairingCardMsg;
    type Output = PairingCardOutput;

    view! {
        #[root]
        gtk::Box {
            add_css_class: "bluetooth-pairing-card",
            set_orientation: gtk::Orientation::Vertical,
            #[watch]
            set_visible: model.variant != PairingVariant::None,

            #[name = "header"]
            gtk::Box {
                add_css_class: "bluetooth-pairing-header",

                #[name = "header_icon_container"]
                gtk::Box {
                    add_css_class: "bluetooth-device-icon",
                    set_valign: gtk::Align::Center,
                    #[name = "header_icon"]
                    gtk::Image {
                        add_css_class: "bluetooth-icon",
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_icon_name: Some(model.device_icon),
                    },
                },

                #[name = "header_info"]
                gtk::Box {
                    add_css_class: "bluetooth-pairing-device-info",
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,
                    set_valign: gtk::Align::Center,
                    #[name = "header_device_name"]
                    gtk::Label {
                        add_css_class: "bluetooth-device-name",
                        set_halign: gtk::Align::Start,
                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                        #[watch]
                        set_label: &model.device_name,
                    },
                    #[name = "header_device_type"]
                    gtk::Label {
                        add_css_class: "bluetooth-device-detail",
                        set_halign: gtk::Align::Start,
                        #[watch]
                        set_label: &model.device_type,
                    },
                },

                #[template]
                GhostIconButton {
                    add_css_class: "bluetooth-pairing-close",
                    set_icon_name: "ld-x-symbolic",
                    set_valign: gtk::Align::Start,
                    connect_clicked => PairingCardMsg::Cancel,
                },
            },

            #[name = "display_pin_section"]
            gtk::Box {
                add_css_class: "bluetooth-pin-display",
                set_orientation: gtk::Orientation::Vertical,
                #[watch]
                set_visible: model.variant == PairingVariant::DisplayPin,
                gtk::Label {
                    add_css_class: "bluetooth-pin-label",
                    set_label: &t!("dropdown-bluetooth-pairing-enter-pin"),
                },
                gtk::Label {
                    add_css_class: "bluetooth-pin-code",
                    #[watch]
                    set_label: &model.pin_code,
                },
            },

            #[name = "display_pin_hint"]
            gtk::Label {
                add_css_class: "bluetooth-pairing-message",
                #[watch]
                set_visible: model.variant == PairingVariant::DisplayPin,
                set_label: &t!("dropdown-bluetooth-pairing-type-on-device"),
                set_wrap: true,
            },

            #[name = "authorization_message"]
            gtk::Label {
                add_css_class: "bluetooth-pairing-message",
                #[watch]
                set_visible: model.variant == PairingVariant::RequestAuthorization,
                set_label: &t!("dropdown-bluetooth-pairing-allow-pairing"),
                set_wrap: true,
            },

            #[name = "passkey_input_message"]
            gtk::Label {
                add_css_class: "bluetooth-pairing-message",
                #[watch]
                set_visible: model.variant == PairingVariant::RequestPasskey,
                set_label: &t!("dropdown-bluetooth-pairing-enter-shown-pin"),
                set_wrap: true,
            },

            #[name = "pin_input_row"]
            gtk::Box {
                add_css_class: "bluetooth-pin-input-row",
                set_halign: gtk::Align::Center,
                #[watch]
                set_visible: model.variant == PairingVariant::RequestPasskey,

                #[name = "pin_digit_0"]
                gtk::Entry {
                    add_css_class: "bluetooth-pin-digit",
                    set_max_length: 1,
                    set_max_width_chars: 1,
                },
                #[name = "pin_digit_1"]
                gtk::Entry {
                    add_css_class: "bluetooth-pin-digit",
                    set_max_length: 1,
                    set_max_width_chars: 1,
                },
                #[name = "pin_digit_2"]
                gtk::Entry {
                    add_css_class: "bluetooth-pin-digit",
                    set_max_length: 1,
                    set_max_width_chars: 1,
                },
                #[name = "pin_digit_3"]
                gtk::Entry {
                    add_css_class: "bluetooth-pin-digit",
                    set_max_length: 1,
                    set_max_width_chars: 1,
                },
                #[name = "pin_digit_4"]
                gtk::Entry {
                    add_css_class: "bluetooth-pin-digit",
                    set_max_length: 1,
                    set_max_width_chars: 1,
                },
                #[name = "pin_digit_5"]
                gtk::Entry {
                    add_css_class: "bluetooth-pin-digit",
                    set_max_length: 1,
                    set_max_width_chars: 1,
                },
            },

            #[name = "confirmation_message"]
            gtk::Label {
                add_css_class: "bluetooth-pairing-message",
                #[watch]
                set_visible: model.variant == PairingVariant::RequestConfirmation,
                set_label: &t!("dropdown-bluetooth-pairing-confirm-code"),
                set_wrap: true,
            },

            #[name = "confirmation_pin_display"]
            gtk::Box {
                add_css_class: "bluetooth-pin-display",
                set_orientation: gtk::Orientation::Vertical,
                #[watch]
                set_visible: model.variant == PairingVariant::RequestConfirmation,
                gtk::Label {
                    add_css_class: "bluetooth-pin-code",
                    #[watch]
                    set_label: &model.pin_code,
                },
            },

            #[name = "display_passkey_section"]
            gtk::Box {
                add_css_class: "bluetooth-pin-display",
                set_orientation: gtk::Orientation::Vertical,
                #[watch]
                set_visible: model.variant == PairingVariant::DisplayPasskey,
                gtk::Label {
                    add_css_class: "bluetooth-pin-label",
                    set_label: &t!("dropdown-bluetooth-pairing-enter-pin"),
                },
                gtk::Label {
                    add_css_class: "bluetooth-pin-code",
                    #[watch]
                    set_label: &model.pin_code,
                },
                #[name = "progress_dots"]
                gtk::Box { add_css_class: "bluetooth-progress-dots" },
            },

            #[name = "passkey_progress_message"]
            gtk::Label {
                add_css_class: "bluetooth-pairing-message",
                #[watch]
                set_visible: model.variant == PairingVariant::DisplayPasskey,
                #[watch]
                set_label: &t!(
                    "dropdown-bluetooth-pairing-entering",
                    entered = model.passkey_entered,
                    total = model.passkey_total
                ),
                set_wrap: true,
            },

            #[name = "service_auth_info"]
            gtk::Box {
                add_css_class: "bluetooth-service-info",
                set_orientation: gtk::Orientation::Vertical,
                #[watch]
                set_visible: model.variant == PairingVariant::RequestServiceAuthorization,
                gtk::Label {
                    add_css_class: "bluetooth-service-name",
                    set_halign: gtk::Align::Start,
                    #[watch]
                    set_label: &model.service_uuid,
                },
            },

            #[name = "service_auth_message"]
            gtk::Label {
                add_css_class: "bluetooth-pairing-message",
                #[watch]
                set_visible: model.variant == PairingVariant::RequestServiceAuthorization,
                set_label: &t!("dropdown-bluetooth-pairing-service-allow"),
                set_wrap: true,
            },

            #[name = "legacy_pin_message"]
            gtk::Label {
                add_css_class: "bluetooth-pairing-message",
                #[watch]
                set_visible: model.variant == PairingVariant::RequestPinCode,
                set_label: &t!("dropdown-bluetooth-pairing-enter-legacy-pin"),
                set_wrap: true,
            },

            #[name = "legacy_pin_entry"]
            gtk::Entry {
                add_css_class: "bluetooth-legacy-pin-input",
                #[watch]
                set_visible: model.variant == PairingVariant::RequestPinCode,
                set_max_length: 16,
                set_placeholder_text: Some(&t!("dropdown-bluetooth-pairing-pin-placeholder")),
            },

            #[name = "legacy_pin_hint"]
            gtk::Label {
                add_css_class: "bluetooth-pin-hint",
                #[watch]
                set_visible: model.variant == PairingVariant::RequestPinCode,
                set_label: &t!("dropdown-bluetooth-pairing-common-pins"),
            },

            #[name = "error_section"]
            gtk::Box {
                add_css_class: "bluetooth-pairing-error",
                #[watch]
                set_visible: model.variant == PairingVariant::Failed,
                gtk::Image {
                    add_css_class: "bluetooth-pairing-error-icon",
                    set_icon_name: Some("ld-x-circle-symbolic"),
                },
                gtk::Label {
                    add_css_class: "bluetooth-pairing-error-text",
                    set_label: &t!("dropdown-bluetooth-pairing-failed"),
                    set_wrap: true,
                    set_hexpand: true,
                },
            },

            #[name = "action_buttons"]
            gtk::Box {
                add_css_class: "bluetooth-pairing-actions",
                set_homogeneous: true,

                #[template]
                GhostButton {
                    set_halign: gtk::Align::Fill,
                    #[template_child]
                    label {
                        set_hexpand: true,
                        set_halign: gtk::Align::Center,
                        #[watch]
                        set_label: &model.left_action_label(),
                    },
                    connect_clicked => PairingCardMsg::Reject,
                },

                #[template]
                PrimaryButton {
                    set_halign: gtk::Align::Fill,
                    #[watch]
                    set_visible: model.has_confirm_action(),
                    #[template_child]
                    label {
                        set_hexpand: true,
                        set_halign: gtk::Align::Center,
                        #[watch]
                        set_label: &model.right_action_label(),
                    },
                    connect_clicked => PairingCardMsg::Confirm,
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = Self {
            variant: PairingVariant::None,
            device_name: String::new(),
            device_type: String::new(),
            device_icon: "ld-bluetooth-symbolic",
            pin_code: String::new(),
            passkey_entered: 0,
            passkey_total: 6,
            service_uuid: String::new(),
            legacy_pin: String::new(),
            pin_entries: Default::default(),
            legacy_pin_entry: gtk::Entry::default(),
        };

        let widgets = view_output!();

        model.legacy_pin_entry = widgets.legacy_pin_entry.clone();
        model.pin_entries = [
            widgets.pin_digit_0.clone(),
            widgets.pin_digit_1.clone(),
            widgets.pin_digit_2.clone(),
            widgets.pin_digit_3.clone(),
            widgets.pin_digit_4.clone(),
            widgets.pin_digit_5.clone(),
        ];

        methods::setup_pin_entries(model.pin_entries.clone());

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            PairingCardMsg::SetRequest {
                request,
                device_name,
                device_icon,
                device_type_key,
            } => {
                self.device_name = device_name;
                self.device_icon = device_icon;
                self.device_type = td!(device_type_key);
                self.apply_request(&request);
            }

            PairingCardMsg::Clear => {
                self.variant = PairingVariant::None;
            }

            PairingCardMsg::Confirm => {
                if let Some(output) = self.build_confirm_output() {
                    let _ = sender.output(output);
                }
            }

            PairingCardMsg::Reject => {
                let _ = sender.output(self.build_reject_output());
            }

            PairingCardMsg::Cancel => {
                let _ = sender.output(PairingCardOutput::Cancelled);
            }
        }
    }
}
