use gtk::prelude::*;
use relm4::{
    gtk,
    gtk::{gdk, glib},
};
use wayle_bluetooth::types::agent::PairingRequest;

use super::{PairingCard, PairingVariant};
use crate::{
    i18n::{t, td},
    shell::bar::dropdowns::bluetooth::{
        helpers::{format_passkey, service_name_key},
        messages::PairingCardOutput,
    },
};

enum PinKeyAction {
    Digit(char),
    Backspace,
    PassThrough,
}

impl PairingCard {
    fn clear_inputs(&mut self) {
        for entry in &self.pin_entries {
            entry.set_text("");
        }
        self.legacy_pin_entry.set_text("");
        self.pin_code.clear();
        self.legacy_pin.clear();
    }

    pub(super) fn apply_request(&mut self, request: &PairingRequest) {
        self.clear_inputs();
        match request {
            PairingRequest::DisplayPinCode { pincode, .. } => {
                self.variant = PairingVariant::DisplayPin;
                self.pin_code = pincode.clone();
            }
            PairingRequest::RequestPasskey { .. } => {
                self.variant = PairingVariant::RequestPasskey;
                self.pin_code.clear();
            }
            PairingRequest::DisplayPasskey {
                passkey, entered, ..
            } => {
                self.variant = PairingVariant::DisplayPasskey;
                self.pin_code = format_passkey(*passkey);
                self.passkey_entered = *entered;
            }
            PairingRequest::RequestConfirmation { passkey, .. } => {
                self.variant = PairingVariant::RequestConfirmation;
                self.pin_code = format_passkey(*passkey);
            }
            PairingRequest::RequestAuthorization { .. } => {
                self.variant = PairingVariant::RequestAuthorization;
            }
            PairingRequest::RequestServiceAuthorization { uuid, .. } => {
                self.variant = PairingVariant::RequestServiceAuthorization;
                let name_key = service_name_key(uuid);
                self.service_uuid = td!(name_key);
            }
            PairingRequest::RequestPinCode { .. } => {
                self.variant = PairingVariant::RequestPinCode;
                self.legacy_pin.clear();
            }
        }
    }

    pub(super) fn left_action_label(&self) -> String {
        match self.variant {
            PairingVariant::RequestConfirmation | PairingVariant::RequestPasskey => {
                t!("dropdown-bluetooth-reject")
            }
            PairingVariant::RequestAuthorization | PairingVariant::RequestServiceAuthorization => {
                t!("dropdown-bluetooth-deny")
            }
            PairingVariant::Failed => {
                t!("dropdown-bluetooth-cancel")
            }
            _ => t!("dropdown-bluetooth-cancel"),
        }
    }

    pub(super) fn right_action_label(&self) -> String {
        match self.variant {
            PairingVariant::RequestPasskey | PairingVariant::RequestPinCode => {
                t!("dropdown-bluetooth-pair")
            }
            PairingVariant::RequestConfirmation => {
                t!("dropdown-bluetooth-confirm")
            }
            PairingVariant::RequestAuthorization => {
                t!("dropdown-bluetooth-allow")
            }
            PairingVariant::RequestServiceAuthorization => {
                t!("dropdown-bluetooth-allow")
            }
            PairingVariant::Failed => {
                t!("dropdown-bluetooth-try-again")
            }
            _ => t!("dropdown-bluetooth-confirm"),
        }
    }

    pub(super) fn has_confirm_action(&self) -> bool {
        !matches!(
            self.variant,
            PairingVariant::DisplayPin | PairingVariant::DisplayPasskey | PairingVariant::None
        )
    }

    pub(super) fn build_confirm_output(&self) -> Option<PairingCardOutput> {
        match self.variant {
            PairingVariant::RequestPasskey => {
                let passkey: String = self
                    .pin_entries
                    .iter()
                    .map(|entry| entry.text().to_string())
                    .collect();
                Some(PairingCardOutput::PinSubmitted(passkey))
            }
            PairingVariant::RequestConfirmation => Some(PairingCardOutput::PasskeyConfirmed),
            PairingVariant::RequestAuthorization => Some(PairingCardOutput::AuthorizationAccepted),
            PairingVariant::RequestServiceAuthorization => {
                Some(PairingCardOutput::ServiceAuthorizationAccepted)
            }
            PairingVariant::RequestPinCode => Some(PairingCardOutput::LegacyPinSubmitted(
                self.legacy_pin_entry.text().to_string(),
            )),
            _ => None,
        }
    }

    pub(super) fn build_reject_output(&self) -> PairingCardOutput {
        match self.variant {
            PairingVariant::RequestConfirmation => PairingCardOutput::PasskeyRejected,
            PairingVariant::RequestAuthorization => PairingCardOutput::AuthorizationRejected,
            PairingVariant::RequestServiceAuthorization => {
                PairingCardOutput::ServiceAuthorizationRejected
            }
            _ => PairingCardOutput::Cancelled,
        }
    }
}

fn classify_key(key: gdk::Key) -> Option<PinKeyAction> {
    match key {
        gdk::Key::_0 | gdk::Key::KP_0 => Some(PinKeyAction::Digit('0')),
        gdk::Key::_1 | gdk::Key::KP_1 => Some(PinKeyAction::Digit('1')),
        gdk::Key::_2 | gdk::Key::KP_2 => Some(PinKeyAction::Digit('2')),
        gdk::Key::_3 | gdk::Key::KP_3 => Some(PinKeyAction::Digit('3')),
        gdk::Key::_4 | gdk::Key::KP_4 => Some(PinKeyAction::Digit('4')),
        gdk::Key::_5 | gdk::Key::KP_5 => Some(PinKeyAction::Digit('5')),
        gdk::Key::_6 | gdk::Key::KP_6 => Some(PinKeyAction::Digit('6')),
        gdk::Key::_7 | gdk::Key::KP_7 => Some(PinKeyAction::Digit('7')),
        gdk::Key::_8 | gdk::Key::KP_8 => Some(PinKeyAction::Digit('8')),
        gdk::Key::_9 | gdk::Key::KP_9 => Some(PinKeyAction::Digit('9')),
        gdk::Key::BackSpace => Some(PinKeyAction::Backspace),
        gdk::Key::Tab | gdk::Key::ISO_Left_Tab => Some(PinKeyAction::PassThrough),
        _ => None,
    }
}

fn handle_pin_key(entries: &[gtk::Entry; 6], index: usize, key: gdk::Key) -> glib::Propagation {
    let Some(action) = classify_key(key) else {
        return glib::Propagation::Stop;
    };

    match action {
        PinKeyAction::Digit(ch) => {
            entries[index].set_text(&ch.to_string());
            if index < entries.len() - 1 {
                entries[index + 1].grab_focus();
            }
            glib::Propagation::Stop
        }

        PinKeyAction::Backspace => {
            if entries[index].text().is_empty() && index > 0 {
                entries[index - 1].set_text("");
                entries[index - 1].grab_focus();
            } else {
                entries[index].set_text("");
            }
            glib::Propagation::Stop
        }

        PinKeyAction::PassThrough => glib::Propagation::Proceed,
    }
}

pub(super) fn setup_pin_entries(entries: [gtk::Entry; 6]) {
    for (index, entry) in entries.iter().enumerate() {
        let entries = entries.clone();
        let key_controller = gtk::EventControllerKey::new();
        key_controller.set_propagation_phase(gtk::PropagationPhase::Capture);
        key_controller
            .connect_key_pressed(move |_, key, _, _| handle_pin_key(&entries, index, key));
        entry.add_controller(key_controller);
    }
}
