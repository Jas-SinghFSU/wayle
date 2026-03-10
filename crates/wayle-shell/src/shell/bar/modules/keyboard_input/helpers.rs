use serde_json::json;
use std::collections::HashMap;
use wayle_hyprland::DeviceInfo;

pub(super) fn format_label(
    layout: &str,
    format: &str,
    language_name_map: &HashMap<String, String>,
) -> String {
    let ctx = json!({ "layout": layout });
    let raw = wayle_common::template::render(format, ctx).unwrap_or_default();
    return language_name_map.get(&raw).unwrap_or(&raw).to_string();
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
        assert_eq!(format_label("us", "{{ layout }}", &HashMap::new()), "us");
    }

    #[test]
    fn format_with_prefix() {
        assert_eq!(
            format_label("de", "KB: {{ layout }}", &HashMap::new()),
            "KB: de"
        );
    }

    #[test]
    fn format_multiple_placeholders() {
        assert_eq!(
            format_label("us", "{{ layout }} | {{ layout }}", &HashMap::new()),
            "us | us"
        );
    }

    #[test]
    fn format_lang_name_map_match() {
        assert_eq!(
            format_label(
                "us",
                "{{ layout }}",
                &HashMap::from([
                    ("us".to_string(), "US".to_string()),
                    ("de".to_string(), "DE".to_string()),
                ])
            ),
            "US",
        );
    }

    #[test]
    fn format_lang_name_map_no_match() {
        assert_eq!(
            format_label(
                "cz",
                "{{ layout }}",
                &HashMap::from([
                    ("us".to_string(), "US".to_string()),
                    ("de".to_string(), "DE".to_string()),
                ])
            ),
            "cz",
        );
    }
}
