use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::Property;

/// Styling configuration for button UI components.
///
/// Defines visual properties for buttons used throughout the Wayle interface,
/// including colors, spacing, and border styling.
/// Each field is reactive and can be watched for changes.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ButtonStyling {
    /// Button background color
    pub background: Property<String>,

    /// Icon color
    pub icon_color: Property<String>,

    /// Corner roundness where higher value represents more rounding
    pub border_radius: Property<u8>,

    /// Internal spacing in (px|em|rem)
    pub padding: Property<String>,
}

/// Styling configuration for dropdown UI components.
///
/// Defines visual properties for dropdown menus used in the Wayle interface,
/// including colors and border styling.
/// Each field is reactive and can be watched for changes.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DropdownStyling {
    /// Dropdown background color
    pub background: Property<String>,

    /// Text color
    pub text_color: Property<String>,

    /// Corner roundness where higher value represents more rounding
    pub border_radius: Property<u8>,
}
