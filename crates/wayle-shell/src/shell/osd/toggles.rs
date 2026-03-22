use std::time::Duration;

use evdev::{EventStream, EventType, KeyCode, LedCode};
use tracing::{info, warn};

use super::messages::ToggleKey;

const LED_READ_DELAY: Duration = Duration::from_millis(50);

pub(super) fn find_keyboards() -> Vec<EventStream> {
    let streams: Vec<EventStream> = evdev::enumerate()
        .filter_map(|(_, device)| {
            let supports_keys = device.supported_events().contains(EventType::KEY);
            let supports_leds = device.supported_events().contains(EventType::LED);

            if !supports_keys || !supports_leds {
                return None;
            }

            match device.into_event_stream() {
                Ok(stream) => Some(stream),

                Err(err) => {
                    warn!(
                        error = %err,
                        "cannot create event stream for keyboard"
                    );
                    None
                }
            }
        })
        .collect();

    if streams.is_empty() {
        warn!(
            "toggle OSD disabled: no keyboard devices accessible, \
             run 'sudo usermod -aG input $USER' and re-login"
        );
    } else {
        info!(
            count = streams.len(),
            "keyboards found for toggle monitoring"
        );
    }

    streams
}

pub(super) fn detect_toggle(event_type: EventType, code: u16, value: i32) -> Option<ToggleKey> {
    if event_type != EventType::KEY || value != 0 {
        return None;
    }

    match KeyCode(code) {
        KeyCode::KEY_CAPSLOCK => Some(ToggleKey::CapsLock),
        KeyCode::KEY_NUMLOCK => Some(ToggleKey::NumLock),
        KeyCode::KEY_SCROLLLOCK => Some(ToggleKey::ScrollLock),
        _ => None,
    }
}

pub(super) fn read_led_state(stream: &EventStream, key: ToggleKey) -> bool {
    let led_code = match key {
        ToggleKey::CapsLock => LedCode::LED_CAPSL,
        ToggleKey::NumLock => LedCode::LED_NUML,
        ToggleKey::ScrollLock => LedCode::LED_SCROLLL,
    };

    stream
        .device()
        .get_led_state()
        .is_ok_and(|leds| leds.contains(led_code))
}

pub(super) const LED_DELAY: Duration = LED_READ_DELAY;
