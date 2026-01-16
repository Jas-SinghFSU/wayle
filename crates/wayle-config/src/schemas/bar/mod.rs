mod types;

pub use types::{BarGroup, BarItem, BarLayout, BarModule, BorderLocation};

use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use crate::schemas::bar::types::Location;

use super::styling::{ColorValue, PaletteColor};

/// Bar configuration.
#[wayle_config]
pub struct BarConfig {
    /// Per-monitor bar layouts.
    #[default(vec![BarLayout::default()])]
    pub layout: ConfigProperty<Vec<BarLayout>>,

    /// Bar-specific scale multiplier for spacing, radius, and other bar elements.
    #[default(1.0)]
    pub scale: ConfigProperty<f32>,

    /// Detach bar from screen edge with margins around it.
    #[default(false)]
    pub floating: ConfigProperty<bool>,

    /// Bar position on screen edge.
    #[default(Location::Top)]
    pub location: ConfigProperty<Location>,

    /// Bar background color.
    #[default(ColorValue::Palette(PaletteColor::Primary))]
    pub bg: ConfigProperty<ColorValue>,

    /// Scale multiplier for button icon size. Range: 0.25–3.0.
    #[serde(rename = "button-icon-scale")]
    #[default(1.0)]
    pub button_icon_scale: ConfigProperty<f32>,

    /// Scale multiplier for button icon container padding. Range: 0.25–3.0.
    #[serde(rename = "button-icon-padding-scale")]
    #[default(1.0)]
    pub button_icon_padding_scale: ConfigProperty<f32>,

    /// Scale multiplier for button label text size. Range: 0.25–3.0.
    #[serde(rename = "button-label-scale")]
    #[default(1.0)]
    pub button_label_scale: ConfigProperty<f32>,

    /// Scale multiplier for button label container padding. Range: 0.25–3.0.
    #[serde(rename = "button-label-padding-scale")]
    #[default(1.0)]
    pub button_label_padding_scale: ConfigProperty<f32>,

    /// Scale multiplier for gap between icon and label. Range: 0.25–3.0.
    #[serde(rename = "button-gap-scale")]
    #[default(1.0)]
    pub button_gap_scale: ConfigProperty<f32>,

    /// Border placement for bar buttons.
    #[serde(rename = "button-border-location")]
    #[default(BorderLocation::None)]
    pub button_border_location: ConfigProperty<BorderLocation>,

    /// Border width for bar buttons (pixels).
    #[serde(rename = "button-border-width")]
    #[default(1u8)]
    pub button_border_width: ConfigProperty<u8>,

    /// Whether or not to enable the shadow for the bar
    #[serde(rename = "shadow-enabled")]
    #[default(false)]
    pub shadow_enabled: ConfigProperty<bool>,

    /// Scale multiplier for dropdown panels spawned from bar modules.
    #[serde(rename = "dropdown-scale")]
    #[default(1.0)]
    pub dropdown_scale: ConfigProperty<f32>,

    /// Whether or not to enable the shadow for the dropdown menus
    #[serde(rename = "dropdown-shadow-enabled")]
    #[default(false)]
    pub dropdown_shadow_enabled: ConfigProperty<bool>,
}
