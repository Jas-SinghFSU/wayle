use serde_json::json;
use wayle_hyprland::DeviceInfo;

pub(super) fn format_label(format: &str, layout: &str) -> String {
    let ctx = json!({ "layout": layout });
    wayle_common::template::render(format, ctx).unwrap_or_default()
}

pub(super) fn main_keyboard_layout(devices: &DeviceInfo) -> Option<&str> {
    devices
        .keyboards
        .iter()
        .find(|kb| kb.main)
        .map(|kb| kb.active_keymap.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_layout_only() {
        assert_eq!(format_label("{{ layout }}", "us"), "us");
    }

    #[test]
    fn format_with_prefix() {
        assert_eq!(format_label("KB: {{ layout }}", "de"), "KB: de");
    }

    #[test]
    fn format_multiple_placeholders() {
        assert_eq!(format_label("{{ layout }} | {{ layout }}", "us"), "us | us");
    }
}
