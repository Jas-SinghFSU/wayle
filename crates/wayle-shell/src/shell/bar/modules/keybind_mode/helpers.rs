use crate::i18n::t;

pub(super) fn format_label(format: &str, mode: &str) -> String {
    let display_mode = if mode.is_empty() {
        t!("bar-keybind-mode-default")
    } else {
        mode.to_string()
    };
    format.replace("{mode}", &display_mode)
}

pub(super) fn compute_visibility(mode: &str, auto_hide: bool) -> bool {
    !auto_hide || !mode.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod format_label {
        use super::*;

        #[test]
        fn placeholder_only() {
            assert_eq!(format_label("{mode}", "resize"), "resize");
        }

        #[test]
        fn with_prefix() {
            assert_eq!(format_label("Mode: {mode}", "resize"), "Mode: resize");
        }

        #[test]
        fn empty_mode_shows_default() {
            assert_eq!(format_label("{mode}", ""), t!("bar-keybind-mode-default"));
        }

        #[test]
        fn no_placeholder() {
            assert_eq!(format_label("Static", "resize"), "Static");
        }
    }

    mod compute_visibility {
        use super::*;

        #[test]
        fn auto_hide_disabled_empty_mode() {
            assert!(compute_visibility("", false));
        }

        #[test]
        fn auto_hide_disabled_active_mode() {
            assert!(compute_visibility("resize", false));
        }

        #[test]
        fn auto_hide_enabled_empty_mode() {
            assert!(!compute_visibility("", true));
        }

        #[test]
        fn auto_hide_enabled_active_mode() {
            assert!(compute_visibility("resize", true));
        }
    }
}
