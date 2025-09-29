use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Styling configuration for button UI components.
///
/// Defines visual properties for buttons used throughout the Wayle interface,
/// including colors, spacing, and border styling.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ButtonStyling {
    /// Button background color
    pub background: String,

    /// Icon color
    pub icon_color: String,

    /// Corner roundness where higher value represents more rounding
    pub border_radius: u8,

    /// Internal spacing in (px|em|rem)
    pub padding: String,
}

/// Styling configuration for dropdown UI components.
///
/// Defines visual properties for dropdown menus used in the Wayle interface,
/// including colors and border styling.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DropdownStyling {
    /// Dropdown background color
    pub background: String,

    /// Text color
    pub text_color: String,

    /// Corner roundness where higher value represents more rounding
    pub border_radius: u8,
}
