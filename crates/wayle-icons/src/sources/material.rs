//! Material Design Icons source.
//!
//! Website: <https://fonts.google.com/icons>

use super::IconSource;

/// Material Design Icons from Google.
///
/// Standard baseline icons from Google's Material Design system.
#[derive(Debug, Clone, Copy, Default)]
pub struct Material;

impl IconSource for Material {
    fn display_name(&self) -> &'static str {
        "Material Design"
    }

    fn cli_name(&self) -> &'static str {
        "material"
    }

    fn prefix(&self) -> &'static str {
        "md"
    }

    fn description(&self) -> &'static str {
        "Google Material Design icons"
    }

    fn website(&self) -> &'static str {
        "https://fonts.google.com/icons"
    }

    fn cdn_url(&self, slug: &str) -> String {
        format!("https://cdn.jsdelivr.net/npm/@material-symbols/svg-400/outlined/{slug}.svg")
    }
}
