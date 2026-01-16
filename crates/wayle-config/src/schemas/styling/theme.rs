use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use crate::infrastructure::themes::{palettes::catppuccin, Palette};

use super::types::ThemeEntry;

/// Color palette configuration.
#[wayle_config]
pub struct ThemeConfig {
    /// The active color palette.
    #[default(catppuccin())]
    pub active: ConfigProperty<Palette>,

    /// Available themes discovered at runtime.
    #[serde(skip)]
    #[schemars(skip)]
    #[wayle(skip)]
    #[default(Vec::new())]
    pub available: ConfigProperty<Vec<ThemeEntry>>,
}
