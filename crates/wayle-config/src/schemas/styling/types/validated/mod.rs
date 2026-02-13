//! Validated newtypes with built-in constraints.

mod hex_color;
mod normalized;
mod percentage;
mod scale;
mod spacing;

pub use hex_color::{HexColor, InvalidHexColor};
pub use normalized::NormalizedF64;
pub use percentage::Percentage;
pub use scale::ScaleFactor;
pub use spacing::Spacing;
