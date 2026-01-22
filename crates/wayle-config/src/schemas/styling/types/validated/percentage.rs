//! Percentage newtype (0-100).

use std::{
    fmt::{self, Display, Formatter},
    ops::Deref,
};

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use tracing::warn;

const MAX: u8 = 100;

/// Percentage value clamped to 0-100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(transparent)]
#[schemars(transparent)]
#[derive(Default)]
pub struct Percentage(#[schemars(range(min = 0, max = MAX))] u8);

impl Percentage {
    /// Creates a percentage value, clamping to 0-100.
    #[must_use]
    pub fn new(value: u8) -> Self {
        Self(value.min(MAX))
    }

    /// Returns the inner u8 value.
    #[must_use]
    pub fn value(self) -> u8 {
        self.0
    }
}

impl Deref for Percentage {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Percentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

impl From<u8> for Percentage {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for Percentage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = u8::deserialize(deserializer)?;
        if raw > MAX {
            warn!("percentage {} exceeds maximum, clamped to {}", raw, MAX);
        }
        Ok(Self::new(raw))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_above_max() {
        assert_eq!(Percentage::new(150).value(), MAX);
        assert_eq!(Percentage::new(255).value(), MAX);
    }

    #[test]
    fn preserves_valid() {
        assert_eq!(Percentage::new(0).value(), 0);
        assert_eq!(Percentage::new(50).value(), 50);
        assert_eq!(Percentage::new(MAX).value(), MAX);
    }

    #[test]
    fn display_includes_percent_sign() {
        assert_eq!(format!("{}", Percentage::new(42)), "42%");
    }
}
