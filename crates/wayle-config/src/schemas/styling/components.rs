use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use super::{GapClass, IconSizeClass, PaddingClass, TextSizeClass};

/// Sizing for Basic bar button variant (icon + label, no container backgrounds).
#[wayle_config]
pub struct BasicButtonSizing {
    /// Icon size class.
    #[serde(rename = "icon-size")]
    #[default(IconSizeClass::default())]
    pub icon_size: ConfigProperty<IconSizeClass>,

    /// Label text size class.
    #[serde(rename = "label-size")]
    #[default(TextSizeClass::default())]
    pub label_size: ConfigProperty<TextSizeClass>,

    /// Horizontal button padding.
    #[serde(rename = "padding-x")]
    #[default(PaddingClass::Md)]
    pub padding_x: ConfigProperty<PaddingClass>,

    /// Vertical button padding.
    #[serde(rename = "padding-y")]
    #[default(PaddingClass::Sm)]
    pub padding_y: ConfigProperty<PaddingClass>,

    /// Gap between icon and label.
    #[default(GapClass::default())]
    pub gap: ConfigProperty<GapClass>,
}

/// Sizing for BlockPrefix bar button variant (icon container bleeds to edge).
#[wayle_config]
pub struct BlockPrefixSizing {
    /// Icon size class.
    #[serde(rename = "icon-size")]
    #[default(IconSizeClass::default())]
    pub icon_size: ConfigProperty<IconSizeClass>,

    /// Horizontal icon container padding.
    #[serde(rename = "icon-padding-x")]
    #[default(PaddingClass::Md)]
    pub icon_padding_x: ConfigProperty<PaddingClass>,

    /// Vertical icon container padding.
    #[serde(rename = "icon-padding-y")]
    #[default(PaddingClass::Sm)]
    pub icon_padding_y: ConfigProperty<PaddingClass>,

    /// Label text size class.
    #[serde(rename = "label-size")]
    #[default(TextSizeClass::default())]
    pub label_size: ConfigProperty<TextSizeClass>,

    /// Horizontal label container padding.
    #[serde(rename = "label-padding-x")]
    #[default(PaddingClass::Md)]
    pub label_padding_x: ConfigProperty<PaddingClass>,

    /// Vertical label container padding.
    #[serde(rename = "label-padding-y")]
    #[default(PaddingClass::Sm)]
    pub label_padding_y: ConfigProperty<PaddingClass>,

    /// Gap between icon container and label.
    #[default(GapClass::Xs)]
    pub gap: ConfigProperty<GapClass>,
}

/// Sizing for IconSquare bar button variant (icon in colored square container).
#[wayle_config]
pub struct IconSquareSizing {
    /// Icon size class.
    #[serde(rename = "icon-size")]
    #[default(IconSizeClass::default())]
    pub icon_size: ConfigProperty<IconSizeClass>,

    /// Horizontal icon container padding.
    #[serde(rename = "icon-padding-x")]
    #[default(PaddingClass::Sm)]
    pub icon_padding_x: ConfigProperty<PaddingClass>,

    /// Vertical icon container padding.
    #[serde(rename = "icon-padding-y")]
    #[default(PaddingClass::Sm)]
    pub icon_padding_y: ConfigProperty<PaddingClass>,

    /// Label text size class.
    #[serde(rename = "label-size")]
    #[default(TextSizeClass::default())]
    pub label_size: ConfigProperty<TextSizeClass>,

    /// Horizontal button padding.
    #[serde(rename = "padding-x")]
    #[default(PaddingClass::Sm)]
    pub padding_x: ConfigProperty<PaddingClass>,

    /// Vertical button padding.
    #[serde(rename = "padding-y")]
    #[default(PaddingClass::Sm)]
    pub padding_y: ConfigProperty<PaddingClass>,

    /// Gap between icon and label.
    #[default(GapClass::default())]
    pub gap: ConfigProperty<GapClass>,
}
