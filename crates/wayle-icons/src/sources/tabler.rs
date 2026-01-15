//! Tabler Icons source - primary UI icons.
//!
//! Website: <https://tabler.io/icons>
//!
//! Two variants available:
//! - `Tabler` (outline) - Stroke-based icons, not GTK symbolic compatible
//! - `TablerFilled` - Fill-based icons, GTK symbolic compatible

use super::IconSource;

/// Tabler Icons outline variant.
///
/// These are stroke-based icons. [`TablerFilled`] provides fill-based
/// icons that work better with GTK's symbolic icon CSS color recoloring.
#[derive(Debug, Clone, Copy, Default)]
pub struct Tabler;

impl IconSource for Tabler {
    fn display_name(&self) -> &'static str {
        "Tabler Icons (Outline)"
    }

    fn cli_name(&self) -> &'static str {
        "tabler"
    }

    fn prefix(&self) -> &'static str {
        "tb"
    }

    fn description(&self) -> &'static str {
        "UI icons with outline/stroke style (home, settings, bell)"
    }

    fn website(&self) -> &'static str {
        "https://tabler.io/icons"
    }

    fn cdn_url(&self, slug: &str) -> String {
        format!("https://unpkg.com/@tabler/icons@latest/icons/outline/{slug}.svg")
    }
}

/// Tabler Icons filled variant.
///
/// These are fill-based icons compatible with GTK's symbolic icon system.
/// When installed with `-symbolic` suffix, CSS `color` property will recolor them.
#[derive(Debug, Clone, Copy, Default)]
pub struct TablerFilled;

impl IconSource for TablerFilled {
    fn display_name(&self) -> &'static str {
        "Tabler Icons (Filled)"
    }

    fn cli_name(&self) -> &'static str {
        "tabler-filled"
    }

    fn prefix(&self) -> &'static str {
        "tbf"
    }

    fn description(&self) -> &'static str {
        "UI icons with solid/filled style (home, settings, bell)"
    }

    fn website(&self) -> &'static str {
        "https://tabler.io/icons"
    }

    fn cdn_url(&self, slug: &str) -> String {
        format!("https://unpkg.com/@tabler/icons@latest/icons/filled/{slug}.svg")
    }
}
