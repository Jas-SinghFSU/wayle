use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

/// How a wallpaper image should be scaled to fit the display.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FitMode {
    /// Scale to cover entire display, cropping excess.
    #[default]
    Fill,
    /// Scale to fit within display, showing letterbox bars if needed.
    Fit,
    /// Display at original size, centered.
    Center,
    /// Stretch to exactly fill the display, ignoring aspect ratio.
    Stretch,
}

impl Display for FitMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = match self {
            Self::Fill => "fill",
            Self::Fit => "fit",
            Self::Center => "center",
            Self::Stretch => "stretch",
        };
        write!(f, "{s}")
    }
}

impl FromStr for FitMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fill" => Ok(Self::Fill),
            "fit" => Ok(Self::Fit),
            "center" => Ok(Self::Center),
            "stretch" => Ok(Self::Stretch),
            _ => Err(format!("Invalid fit mode: {s}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_is_case_insensitive() {
        assert_eq!("FILL".parse::<FitMode>().unwrap(), FitMode::Fill);
        assert_eq!("Fill".parse::<FitMode>().unwrap(), FitMode::Fill);
        assert_eq!("fill".parse::<FitMode>().unwrap(), FitMode::Fill);
    }

    #[test]
    fn parse_invalid_returns_descriptive_error() {
        let err = "invalid".parse::<FitMode>().unwrap_err();
        assert!(
            err.contains("invalid"),
            "Error should contain the bad input"
        );
    }
}
