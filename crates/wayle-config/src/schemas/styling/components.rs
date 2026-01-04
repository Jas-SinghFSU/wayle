use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::ConfigProperty;
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

use super::{GapClass, IconSizeClass, PaddingClass, TextSizeClass};

/// Styling configuration for button UI components.
///
/// Defines visual properties for buttons used throughout the Wayle interface,
/// including colors, spacing, and border styling.
/// Each field is reactive and can be watched for changes.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ButtonStyling {
    /// Button background color
    pub background: ConfigProperty<String>,

    /// Icon color
    pub icon_color: ConfigProperty<String>,

    /// Corner roundness where higher value represents more rounding
    pub border_radius: ConfigProperty<u8>,

    /// Internal spacing in (px|em|rem)
    pub padding: ConfigProperty<String>,
}

/// Styling configuration for dropdown UI components.
///
/// Defines visual properties for dropdown menus used in the Wayle interface,
/// including colors and border styling.
/// Each field is reactive and can be watched for changes.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DropdownStyling {
    /// Dropdown background color
    pub background: ConfigProperty<String>,

    /// Text color
    pub text_color: ConfigProperty<String>,

    /// Corner roundness where higher value represents more rounding
    pub border_radius: ConfigProperty<u8>,
}

/// Global sizing for Basic bar button variant.
///
/// Simple icon + label with no container backgrounds. Colors are passed
/// per-module; this config controls consistent sizing across all modules.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    JsonSchema,
    ApplyConfigLayer,
    ApplyRuntimeLayer,
    ExtractRuntimeValues,
    SubscribeChanges,
)]
#[serde(default)]
pub struct BasicButtonSizing {
    /// Icon size class.
    pub icon_size: ConfigProperty<IconSizeClass>,

    /// Label text size class.
    pub text_size: ConfigProperty<TextSizeClass>,

    /// Horizontal button padding.
    pub padding_x: ConfigProperty<PaddingClass>,

    /// Vertical button padding.
    pub padding_y: ConfigProperty<PaddingClass>,

    /// Gap between icon and label.
    pub gap: ConfigProperty<GapClass>,
}

impl Default for BasicButtonSizing {
    fn default() -> Self {
        Self {
            icon_size: ConfigProperty::new(IconSizeClass::default()),
            text_size: ConfigProperty::new(TextSizeClass::default()),
            padding_x: ConfigProperty::new(PaddingClass::Md),
            padding_y: ConfigProperty::new(PaddingClass::Sm),
            gap: ConfigProperty::new(GapClass::default()),
        }
    }
}

/// Global sizing for BlockPrefix bar button variant.
///
/// Icon in a colored container that bleeds to the button edge. Colors are
/// passed per-module; this config controls consistent sizing across all modules.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    JsonSchema,
    ApplyConfigLayer,
    ApplyRuntimeLayer,
    ExtractRuntimeValues,
    SubscribeChanges,
)]
#[serde(default)]
pub struct BlockPrefixSizing {
    /// Icon size class.
    pub icon_size: ConfigProperty<IconSizeClass>,

    /// Horizontal icon container padding.
    pub icon_padding_x: ConfigProperty<PaddingClass>,

    /// Vertical icon container padding.
    pub icon_padding_y: ConfigProperty<PaddingClass>,

    /// Label text size class.
    pub text_size: ConfigProperty<TextSizeClass>,

    /// Horizontal label container padding.
    pub label_padding_x: ConfigProperty<PaddingClass>,

    /// Vertical label container padding.
    pub label_padding_y: ConfigProperty<PaddingClass>,

    /// Gap between icon container and label.
    pub gap: ConfigProperty<GapClass>,
}

impl Default for BlockPrefixSizing {
    fn default() -> Self {
        Self {
            icon_size: ConfigProperty::new(IconSizeClass::default()),
            icon_padding_x: ConfigProperty::new(PaddingClass::Md),
            icon_padding_y: ConfigProperty::new(PaddingClass::Sm),
            text_size: ConfigProperty::new(TextSizeClass::default()),
            label_padding_x: ConfigProperty::new(PaddingClass::Md),
            label_padding_y: ConfigProperty::new(PaddingClass::Sm),
            gap: ConfigProperty::new(GapClass::Xs),
        }
    }
}

/// Global sizing for IconSquare bar button variant.
///
/// Icon in a colored square container inside button padding. Colors are
/// passed per-module; this config controls consistent sizing across all modules.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    JsonSchema,
    ApplyConfigLayer,
    ApplyRuntimeLayer,
    ExtractRuntimeValues,
    SubscribeChanges,
)]
#[serde(default)]
pub struct IconSquareSizing {
    /// Icon size class.
    pub icon_size: ConfigProperty<IconSizeClass>,

    /// Horizontal icon container padding.
    pub icon_padding_x: ConfigProperty<PaddingClass>,

    /// Vertical icon container padding.
    pub icon_padding_y: ConfigProperty<PaddingClass>,

    /// Label text size class.
    pub text_size: ConfigProperty<TextSizeClass>,

    /// Horizontal button padding.
    pub padding_x: ConfigProperty<PaddingClass>,

    /// Vertical button padding.
    pub padding_y: ConfigProperty<PaddingClass>,

    /// Gap between icon and label.
    pub gap: ConfigProperty<GapClass>,
}

impl Default for IconSquareSizing {
    fn default() -> Self {
        Self {
            icon_size: ConfigProperty::new(IconSizeClass::default()),
            icon_padding_x: ConfigProperty::new(PaddingClass::Sm),
            icon_padding_y: ConfigProperty::new(PaddingClass::Sm),
            text_size: ConfigProperty::new(TextSizeClass::default()),
            padding_x: ConfigProperty::new(PaddingClass::Sm),
            padding_y: ConfigProperty::new(PaddingClass::Sm),
            gap: ConfigProperty::new(GapClass::default()),
        }
    }
}
