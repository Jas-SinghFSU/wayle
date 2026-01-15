use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use super::{GapClass, IconSizeClass, PaddingClass, TextSizeClass};

/// Global sizing for Basic bar button variant.
///
/// Simple icon + label with no container backgrounds. Colors are passed
/// per-module; this config controls consistent sizing across all modules.
#[wayle_config]
pub struct BasicButtonSizing {
    /// Icon size class.
    #[default(IconSizeClass::default())]
    pub icon_size: ConfigProperty<IconSizeClass>,

    /// Label text size class.
    #[default(TextSizeClass::default())]
    pub text_size: ConfigProperty<TextSizeClass>,

    /// Horizontal button padding.
    #[default(PaddingClass::Md)]
    pub padding_x: ConfigProperty<PaddingClass>,

    /// Vertical button padding.
    #[default(PaddingClass::Sm)]
    pub padding_y: ConfigProperty<PaddingClass>,

    /// Gap between icon and label.
    #[default(GapClass::default())]
    pub gap: ConfigProperty<GapClass>,
}

/// Global sizing for BlockPrefix bar button variant.
///
/// Icon in a colored container that bleeds to the button edge. Colors are
/// passed per-module; this config controls consistent sizing across all modules.
#[wayle_config]
pub struct BlockPrefixSizing {
    /// Icon size class.
    #[default(IconSizeClass::default())]
    pub icon_size: ConfigProperty<IconSizeClass>,

    /// Horizontal icon container padding.
    #[default(PaddingClass::Md)]
    pub icon_padding_x: ConfigProperty<PaddingClass>,

    /// Vertical icon container padding.
    #[default(PaddingClass::Sm)]
    pub icon_padding_y: ConfigProperty<PaddingClass>,

    /// Label text size class.
    #[default(TextSizeClass::default())]
    pub text_size: ConfigProperty<TextSizeClass>,

    /// Horizontal label container padding.
    #[default(PaddingClass::Md)]
    pub label_padding_x: ConfigProperty<PaddingClass>,

    /// Vertical label container padding.
    #[default(PaddingClass::Sm)]
    pub label_padding_y: ConfigProperty<PaddingClass>,

    /// Gap between icon container and label.
    #[default(GapClass::Xs)]
    pub gap: ConfigProperty<GapClass>,
}

/// Global sizing for IconSquare bar button variant.
///
/// Icon in a colored square container inside button padding. Colors are
/// passed per-module; this config controls consistent sizing across all modules.
#[wayle_config]
pub struct IconSquareSizing {
    /// Icon size class.
    #[default(IconSizeClass::default())]
    pub icon_size: ConfigProperty<IconSizeClass>,

    /// Horizontal icon container padding.
    #[default(PaddingClass::Sm)]
    pub icon_padding_x: ConfigProperty<PaddingClass>,

    /// Vertical icon container padding.
    #[default(PaddingClass::Sm)]
    pub icon_padding_y: ConfigProperty<PaddingClass>,

    /// Label text size class.
    #[default(TextSizeClass::default())]
    pub text_size: ConfigProperty<TextSizeClass>,

    /// Horizontal button padding.
    #[default(PaddingClass::Sm)]
    pub padding_x: ConfigProperty<PaddingClass>,

    /// Vertical button padding.
    #[default(PaddingClass::Sm)]
    pub padding_y: ConfigProperty<PaddingClass>,

    /// Gap between icon and label.
    #[default(GapClass::default())]
    pub gap: ConfigProperty<GapClass>,
}
