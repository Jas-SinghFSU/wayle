//! GTK4 CSS hex color newtype.

use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
    ops::Deref,
    str::FromStr,
};

use serde::{Deserialize, Deserializer, Serialize};

/// GTK4 CSS hex color.
///
/// Accepts `#rgb`, `#rgba`, `#rrggbb`, or `#rrggbbaa` formats.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct HexColor(String);

impl schemars::JsonSchema for HexColor {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("HexColor")
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "description": "GTK4 CSS hex color (#rgb, #rgba, #rrggbb, or #rrggbbaa)",
            "type": "string",
            "pattern": "^#([0-9a-fA-F]{3}|[0-9a-fA-F]{4}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$"
        })
    }
}

/// Error when parsing an invalid hex color.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InvalidHexColor {
    /// Color string doesn't start with '#'.
    #[error("hex color must start with '#', got: {0}")]
    MissingHash(String),

    /// Wrong number of hex digits (must be 3, 4, 6, or 8).
    #[error("hex color must have 3, 4, 6, or 8 hex digits after '#', got {1} digits in: {0}")]
    InvalidLength(String, usize),

    /// Non-hexadecimal character found.
    #[error("hex color contains invalid character '{1}' in: {0}")]
    InvalidCharacter(String, char),
}

impl HexColor {
    /// Creates a hex color, validating the GTK4 CSS format.
    ///
    /// # Errors
    ///
    /// Returns error if format is invalid.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidHexColor> {
        let s: String = value.into();
        Self::validate(&s)?;
        Ok(Self(s))
    }

    /// Returns the inner string value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn validate(s: &str) -> Result<(), InvalidHexColor> {
        if !s.starts_with('#') {
            return Err(InvalidHexColor::MissingHash(s.to_owned()));
        }

        let hex_part = &s[1..];
        let len = hex_part.len();

        if !matches!(len, 3 | 4 | 6 | 8) {
            return Err(InvalidHexColor::InvalidLength(s.to_owned(), len));
        }

        for hex_part_char in hex_part.chars() {
            if !hex_part_char.is_ascii_hexdigit() {
                return Err(InvalidHexColor::InvalidCharacter(
                    s.to_owned(),
                    hex_part_char,
                ));
            }
        }

        Ok(())
    }
}

impl Default for HexColor {
    fn default() -> Self {
        Self("#ffffff".to_owned())
    }
}

impl Deref for HexColor {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for HexColor {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for HexColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for HexColor {
    type Err = InvalidHexColor;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod valid_formats {
        use super::*;

        #[test]
        fn accepts_3_digit_rgb() {
            assert!(HexColor::new("#fff").is_ok());
            assert!(HexColor::new("#FFF").is_ok());
            assert!(HexColor::new("#123").is_ok());
            assert!(HexColor::new("#abc").is_ok());
            assert!(HexColor::new("#ABC").is_ok());
            assert!(HexColor::new("#AbC").is_ok());
        }

        #[test]
        fn accepts_4_digit_rgba() {
            assert!(HexColor::new("#ffff").is_ok());
            assert!(HexColor::new("#FFFF").is_ok());
            assert!(HexColor::new("#1234").is_ok());
            assert!(HexColor::new("#abcd").is_ok());
        }

        #[test]
        fn accepts_6_digit_rrggbb() {
            assert!(HexColor::new("#ffffff").is_ok());
            assert!(HexColor::new("#FFFFFF").is_ok());
            assert!(HexColor::new("#112233").is_ok());
            assert!(HexColor::new("#aabbcc").is_ok());
            assert!(HexColor::new("#11111b").is_ok());
        }

        #[test]
        fn accepts_8_digit_rrggbbaa() {
            assert!(HexColor::new("#ffffffff").is_ok());
            assert!(HexColor::new("#FFFFFFFF").is_ok());
            assert!(HexColor::new("#11223344").is_ok());
            assert!(HexColor::new("#aabbccdd").is_ok());
            assert!(HexColor::new("#00000000").is_ok());
        }

        #[test]
        fn preserves_original_case() {
            assert_eq!(HexColor::new("#FFF").unwrap().as_str(), "#FFF");
            assert_eq!(HexColor::new("#fff").unwrap().as_str(), "#fff");
            assert_eq!(HexColor::new("#AbCdEf").unwrap().as_str(), "#AbCdEf");
        }
    }

    mod invalid_formats {
        use super::*;

        #[test]
        fn rejects_missing_hash() {
            let err = HexColor::new("ffffff").unwrap_err();
            assert!(matches!(err, InvalidHexColor::MissingHash(_)));
        }

        #[test]
        fn rejects_empty_string() {
            let err = HexColor::new("").unwrap_err();
            assert!(matches!(err, InvalidHexColor::MissingHash(_)));
        }

        #[test]
        fn rejects_hash_only() {
            let err = HexColor::new("#").unwrap_err();
            assert!(matches!(err, InvalidHexColor::InvalidLength(_, 0)));
        }

        #[test]
        fn rejects_1_digit() {
            let err = HexColor::new("#f").unwrap_err();
            assert!(matches!(err, InvalidHexColor::InvalidLength(_, 1)));
        }

        #[test]
        fn rejects_2_digits() {
            let err = HexColor::new("#ff").unwrap_err();
            assert!(matches!(err, InvalidHexColor::InvalidLength(_, 2)));
        }

        #[test]
        fn rejects_5_digits() {
            let err = HexColor::new("#fffff").unwrap_err();
            assert!(matches!(err, InvalidHexColor::InvalidLength(_, 5)));
        }

        #[test]
        fn rejects_7_digits() {
            let err = HexColor::new("#fffffff").unwrap_err();
            assert!(matches!(err, InvalidHexColor::InvalidLength(_, 7)));
        }

        #[test]
        fn rejects_9_digits() {
            let err = HexColor::new("#fffffffff").unwrap_err();
            assert!(matches!(err, InvalidHexColor::InvalidLength(_, 9)));
        }

        #[test]
        fn rejects_invalid_characters() {
            let err = HexColor::new("#gggggg").unwrap_err();
            assert!(matches!(err, InvalidHexColor::InvalidCharacter(_, 'g')));

            let err = HexColor::new("#zzzzzz").unwrap_err();
            assert!(matches!(err, InvalidHexColor::InvalidCharacter(_, 'z')));

            let err = HexColor::new("#12345g").unwrap_err();
            assert!(matches!(err, InvalidHexColor::InvalidCharacter(_, 'g')));
        }

        #[test]
        fn rejects_spaces() {
            let err = HexColor::new("#fff ff").unwrap_err();
            assert!(matches!(err, InvalidHexColor::InvalidCharacter(_, ' ')));

            let err = HexColor::new("#ffff f f").unwrap_err();
            assert!(matches!(err, InvalidHexColor::InvalidCharacter(_, ' ')));
        }
    }

    mod serde_integration {
        use super::*;

        #[test]
        fn deserializes_valid_hex() {
            let color: HexColor = serde_json::from_str("\"#ffffff\"").unwrap();
            assert_eq!(color.as_str(), "#ffffff");
        }

        #[test]
        fn rejects_invalid_hex_on_deserialize() {
            let result: Result<HexColor, _> = serde_json::from_str("\"ffffff\"");
            assert!(result.is_err());
        }

        #[test]
        fn serializes_to_string() {
            let color = HexColor::new("#11111b").unwrap();
            let json = serde_json::to_string(&color).unwrap();
            assert_eq!(json, "\"#11111b\"");
        }

        #[test]
        fn roundtrips_through_serde() {
            let original = HexColor::new("#AbCdEf").unwrap();
            let json = serde_json::to_string(&original).unwrap();
            let restored: HexColor = serde_json::from_str(&json).unwrap();
            assert_eq!(original, restored);
        }
    }

    mod toml_integration {
        use super::*;

        #[derive(Debug, Deserialize)]
        struct TestConfig {
            color: HexColor,
        }

        #[test]
        fn deserializes_from_toml() {
            let toml_str = "color = \"#11111b\"";
            let config: TestConfig = toml::from_str(toml_str).unwrap();
            assert_eq!(config.color.as_str(), "#11111b");
        }

        #[test]
        fn rejects_invalid_in_toml() {
            let toml_str = "color = \"invalid\"";
            let result: Result<TestConfig, _> = toml::from_str(toml_str);
            assert!(result.is_err());
        }
    }

    mod display_and_deref {
        use super::*;

        #[test]
        fn display_returns_hex_string() {
            let color = HexColor::new("#11111b").unwrap();
            assert_eq!(format!("{}", color), "#11111b");
        }

        #[test]
        fn deref_to_str() {
            let color = HexColor::new("#ffffff").unwrap();
            let s: &str = &color;
            assert_eq!(s, "#ffffff");
        }
    }

    mod gtk4_compatibility {
        use super::*;

        #[test]
        fn catppuccin_mocha_colors_valid() {
            assert!(HexColor::new("#11111b").is_ok());
            assert!(HexColor::new("#1e1e2e").is_ok());
            assert!(HexColor::new("#313244").is_ok());
            assert!(HexColor::new("#cdd6f4").is_ok());
            assert!(HexColor::new("#a6adc8").is_ok());
            assert!(HexColor::new("#f5c2e7").is_ok());
            assert!(HexColor::new("#f38ba8").is_ok());
            assert!(HexColor::new("#f9e2af").is_ok());
            assert!(HexColor::new("#a6e3a1").is_ok());
            assert!(HexColor::new("#89b4fa").is_ok());
        }
    }
}
