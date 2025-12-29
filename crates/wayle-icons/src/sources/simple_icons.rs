//! Simple Icons source - application and brand icons.
//!
//! Website: <https://simpleicons.org>

use super::IconSource;

/// Simple Icons source for application and brand icons.
#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleIcons;

impl IconSource for SimpleIcons {
    fn display_name(&self) -> &'static str {
        "Simple Icons"
    }

    fn cli_name(&self) -> &'static str {
        "simple-icons"
    }

    fn prefix(&self) -> &'static str {
        "si"
    }

    fn description(&self) -> &'static str {
        "Brand and app logos (firefox, spotify, github)"
    }

    fn website(&self) -> &'static str {
        "https://simpleicons.org"
    }

    fn cdn_url(&self, slug: &str) -> String {
        format!("https://unpkg.com/simple-icons@latest/icons/{}.svg", slug)
    }
}
