//! Styling type definitions organized by domain.
//!
//! - [`color`] - Palette colors and color values
//! - [`sizing`] - Icon sizes, padding, and gaps
//! - [`typography`] - Text sizes and font weights
//! - [`rounding`] - Border radius and global rounding

mod color;
mod rounding;
mod sizing;
mod typography;

pub use color::{ColorValue, InvalidPaletteColor, PaletteColor, ThemeProvider};
pub use rounding::{RadiusClass, RoundingLevel};
pub use sizing::{GapClass, IconSizeClass, PaddingClass};
pub use typography::{FontWeightClass, TextSizeClass};
