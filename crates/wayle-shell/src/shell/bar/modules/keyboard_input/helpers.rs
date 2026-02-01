use wayle_hyprland::DeviceInfo;

pub(super) fn format_label(format: &str, layout: &str) -> String {
    format.replace("{layout}", layout)
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
        assert_eq!(format_label("{layout}", "us"), "us");
    }

    #[test]
    fn format_with_prefix() {
        assert_eq!(format_label("KB: {layout}", "de"), "KB: de");
    }

    #[test]
    fn format_multiple_placeholders() {
        assert_eq!(format_label("{layout} | {layout}", "us"), "us | us");
    }
}
