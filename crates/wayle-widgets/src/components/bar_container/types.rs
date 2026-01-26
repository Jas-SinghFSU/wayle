//! Type definitions for bar container component.

use std::fmt::{Debug, Formatter, Result as FmtResult};

use wayle_common::ConfigProperty;
use wayle_config::schemas::{
    bar::BorderLocation,
    styling::{ColorValue, ThemeProvider},
};

/// Colors for bar containers.
#[derive(Clone)]
pub struct BarContainerColors {
    /// Container background.
    pub background: ConfigProperty<ColorValue>,
    /// Border color.
    pub border_color: ConfigProperty<ColorValue>,
}

impl Debug for BarContainerColors {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("BarContainerColors").finish_non_exhaustive()
    }
}

/// Behavioral settings for bar containers.
#[derive(Clone)]
pub struct BarContainerBehavior {
    /// Show the border.
    pub show_border: ConfigProperty<bool>,
    /// Container visibility.
    pub visible: ConfigProperty<bool>,
}

impl Debug for BarContainerBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("BarContainerBehavior")
            .field("visible", &self.visible.get())
            .finish_non_exhaustive()
    }
}

/// Initialization data for BarContainer.
#[derive(Debug, Clone)]
pub struct BarContainerInit {
    /// Container-specific color configuration.
    pub colors: BarContainerColors,
    /// Container-specific behavior configuration.
    pub behavior: BarContainerBehavior,
    /// Vertical orientation (derived from bar location).
    pub is_vertical: ConfigProperty<bool>,
    /// Theme provider for color resolution.
    pub theme_provider: ConfigProperty<ThemeProvider>,
    /// Border width in pixels.
    pub border_width: ConfigProperty<u8>,
    /// Border placement.
    pub border_location: ConfigProperty<BorderLocation>,
}

/// CSS class constants for bar containers.
pub struct BarContainerClass;

impl BarContainerClass {
    /// Base class for bar containers.
    pub const BASE: &'static str = "bar-container";

    /// Applied for vertical bar orientation.
    pub const VERTICAL: &'static str = "vertical";
}
