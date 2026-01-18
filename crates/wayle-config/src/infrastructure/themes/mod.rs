use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Built-in theme palettes.
pub mod palettes;
/// Theme discovery utilities.
pub(crate) mod utils;

/// Ten-color palette for CSS generation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct Palette {
    /// Base background color (darkest).
    pub bg: String,
    /// Card and sidebar background.
    pub surface: String,
    /// Raised element background.
    pub elevated: String,
    /// Primary text color.
    pub fg: String,
    /// Secondary text color.
    pub fg_muted: String,
    /// Accent color for interactive elements.
    pub primary: String,
    /// Red palette color.
    pub red: String,
    /// Yellow palette color.
    pub yellow: String,
    /// Green palette color.
    pub green: String,
    /// Blue palette color.
    pub blue: String,
}
