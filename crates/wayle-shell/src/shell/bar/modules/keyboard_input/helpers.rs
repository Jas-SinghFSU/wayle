use std::collections::HashMap;

use serde_json::json;
use wayle_hyprland::DeviceInfo;

pub(super) fn format_label(
    layout: &str,
    format: &str,
    layout_alias_map: &HashMap<String, String>,
) -> String {
    let alias = layout_alias_map
        .get(layout)
        .map(String::as_str)
        .unwrap_or(layout);
    let ctx = json!({ "layout": layout, "alias": alias });
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
            format_label("us", "{{ layout }} | {{ alias }}", &HashMap::new()),
            "us | us"
        );
    }

    #[test]
    fn format_multiple_placeholders_with_alias() {
        assert_eq!(
            format_label(
                "us",
                "{{ layout }} | {{ alias }}",
                &HashMap::from([("us".to_string(), "US".to_string())])
            ),
            "us | US"
        );
    }

    #[test]
    fn format_lang_name_map_match() {
        assert_eq!(
            format_label(
                "us",
                "{{ alias }}",
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
                "{{ alias }}",
                &HashMap::from([
                    ("us".to_string(), "US".to_string()),
                    ("de".to_string(), "DE".to_string()),
                ])
            ),
            "cz",
        );
    }
}
