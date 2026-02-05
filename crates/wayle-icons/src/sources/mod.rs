//! Icon source definitions for different CDN providers.
//!
//! Each source knows its CDN URL pattern and naming prefix.

mod lucide;
mod material;
mod simple_icons;
mod tabler;

pub use lucide::Lucide;
pub use material::Material;
pub use simple_icons::SimpleIcons;
pub use tabler::{Tabler, TablerFilled};

use crate::error::{Error, Result};

/// Custom icon prefix for user-imported icons.
pub const CUSTOM_PREFIX: &str = "cm";

/// Trait defining an icon source with CDN URL pattern and prefix.
pub trait IconSource: Send + Sync {
    /// Human-readable name of the source (e.g., "Tabler Icons").
    fn display_name(&self) -> &'static str;

    /// CLI identifier for the source (e.g., "tabler").
    fn cli_name(&self) -> &'static str;

    /// Prefix added to icon names (e.g., "tb" for "tb-home").
    fn prefix(&self) -> &'static str;

    /// Brief description of what this source provides.
    fn description(&self) -> &'static str;

    /// Website where users can browse available icons.
    fn website(&self) -> &'static str;

    /// Generates the CDN URL for a given icon slug.
    ///
    /// # Arguments
    ///
    /// * `slug` - The icon identifier (e.g., "home", "settings").
    fn cdn_url(&self, slug: &str) -> String;

    /// Generates the full icon name with prefix.
    ///
    /// # Arguments
    ///
    /// * `slug` - The icon identifier.
    fn icon_name(&self, slug: &str) -> String {
        format!("{}-{}", self.prefix(), slug)
    }
}

/// Returns the icon source for a given CLI name.
///
/// # Arguments
///
/// * `name` - The CLI identifier (e.g., "tabler", "simple-icons", "lucide").
///
/// # Errors
///
/// Returns `Error::InvalidSource` if the name is not recognized.
pub fn from_cli_name(name: &str) -> Result<Box<dyn IconSource>> {
    match name {
        "tabler" => Ok(Box::new(Tabler)),
        "tabler-filled" => Ok(Box::new(TablerFilled)),
        "simple-icons" => Ok(Box::new(SimpleIcons)),
        "material" => Ok(Box::new(Material)),
        "lucide" => Ok(Box::new(Lucide)),
        _ => Err(Error::InvalidSource {
            name: name.to_string(),
        }),
    }
}

/// Returns all available icon sources.
pub fn all() -> Vec<Box<dyn IconSource>> {
    vec![
        Box::new(Tabler),
        Box::new(TablerFilled),
        Box::new(SimpleIcons),
        Box::new(Material),
        Box::new(Lucide),
    ]
}

/// Returns all known icon prefixes including custom.
pub fn all_prefixes() -> Vec<&'static str> {
    let mut prefixes: Vec<_> = all().iter().map(|s| s.prefix()).collect();
    prefixes.push(CUSTOM_PREFIX);
    prefixes
}
