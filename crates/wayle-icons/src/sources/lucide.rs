//! Lucide Icons source - alternative UI icons.
//!
//! Website: <https://lucide.dev>

use super::IconSource;

/// Lucide Icons source as an alternative to Tabler.
#[derive(Debug, Clone, Copy, Default)]
pub struct Lucide;

impl IconSource for Lucide {
    fn display_name(&self) -> &'static str {
        "Lucide Icons"
    }

    fn cli_name(&self) -> &'static str {
        "lucide"
    }

    fn prefix(&self) -> &'static str {
        "ld"
    }

    fn description(&self) -> &'static str {
        "Alternative UI icons, Feather fork (home, settings, bell)"
    }

    fn website(&self) -> &'static str {
        "https://lucide.dev/icons"
    }

    fn cdn_url(&self, slug: &str) -> String {
        format!("https://unpkg.com/lucide-static@latest/icons/{}.svg", slug)
    }
}
