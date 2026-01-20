use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::Location;

/// Shadow style for the bar.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ShadowPreset {
    /// No shadow.
    #[default]
    None,
    /// Directional shadow opposite the anchor edge.
    Drop,
    /// All-around shadow.
    Floating,
}

impl ShadowPreset {
    /// Margin in pixels needed for this shadow to render without clipping.
    pub fn margin_px(self) -> u32 {
        match self {
            Self::None => 0,
            Self::Drop => 16,
            Self::Floating => 16,
        }
    }

    /// CSS box-shadow value based on bar position.
    pub fn css_shadow(self, location: Location) -> &'static str {
        match self {
            Self::None => "none",
            Self::Drop => match location {
                Location::Top => "0 8px 24px rgba(0, 0, 0, 0.5)",
                Location::Bottom => "0 -8px 24px rgba(0, 0, 0, 0.5)",
                Location::Left => "8px 0 24px rgba(0, 0, 0, 0.5)",
                Location::Right => "-8px 0 24px rgba(0, 0, 0, 0.5)",
            },
            Self::Floating => "0 0 32px rgba(0, 0, 0, 0.5)",
        }
    }

    /// Margin in pixels for the edge opposite to the anchor where shadow extends.
    pub fn opposite_margin(self) -> u32 {
        self.margin_px()
    }
}
