mod bind;
mod client;
mod device;
mod layer;
mod monitor;
mod workspace;

use std::fmt::{self, Display};

pub use bind::*;
pub use client::*;
pub use device::*;
pub use layer::*;
pub use monitor::*;
use serde::{Deserialize, Deserializer};
pub(crate) use workspace::WorkspaceData;
pub use workspace::WorkspaceRule;

use crate::Error;

/// Unique identifier for a monitor.
pub type MonitorId = i64;
/// Unique identifier for a workspace.
pub type WorkspaceId = i64;
/// Process identifier.
pub type ProcessId = i32;
/// Focus history identifier.
pub type FocusHistoryId = i32;

/// The type of screencopy share.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreencastOwner {
    /// Monitor share.
    Monitor,
    /// Window share.
    Window,
}

impl TryFrom<&str> for ScreencastOwner {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "0" => Ok(Self::Monitor),
            "1" => Ok(Self::Window),
            _ => Err(Error::InvalidEnumValue {
                type_name: "ScreencastOwner",
                value: value.to_string(),
            }),
        }
    }
}

/// Workspace information for a monitor.
#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct WorkspaceInfo {
    /// Workspace ID.
    pub id: WorkspaceId,
    /// Workspace name.
    pub name: String,
}

/// Window address identifier.
#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(from = "String")]
pub struct Address(String);

impl Address {
    /// Creates a new address from a string.
    pub fn new(address: String) -> Self {
        let normalized = address.strip_prefix("0x").unwrap_or(&address).to_string();
        Self(normalized)
    }

    /// Returns the address as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the address and returns the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Address {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for Address {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

pub(crate) fn deserialize_optional_address<'de, D>(
    deserializer: D,
) -> Result<Option<Address>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s == "0" || s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Address::new(s)))
    }
}

pub(crate) fn deserialize_optional_string<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() { Ok(None) } else { Ok(Some(s)) }
}

/// Cursor position in global layout coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition {
    /// The x-coordinate of the cursor
    pub x: i32,
    /// The y-coordinate of the cursor
    pub y: i32,
}

#[cfg(test)]
#[allow(clippy::panic)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[test]
    fn address_new_strips_0x_prefix() {
        let address = Address::new("0xdeadbeef".to_string());

        assert_eq!(address.as_str(), "deadbeef");
    }

    #[test]
    fn address_new_preserves_address_without_prefix() {
        let address = Address::new("deadbeef".to_string());

        assert_eq!(address.as_str(), "deadbeef");
    }

    #[test]
    fn screencast_owner_try_from_converts_monitor() {
        let result = ScreencastOwner::try_from("0");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ScreencastOwner::Monitor);
    }

    #[test]
    fn screencast_owner_try_from_converts_window() {
        let result = ScreencastOwner::try_from("1");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ScreencastOwner::Window);
    }

    #[test]
    fn screencast_owner_try_from_fails_for_invalid_value() {
        let result = ScreencastOwner::try_from("2");

        assert!(result.is_err());
        let error = result.unwrap_err();
        if let Error::InvalidEnumValue { type_name, value } = error {
            assert_eq!(type_name, "ScreencastOwner");
            assert_eq!(value, "2");
        } else {
            panic!("Expected InvalidEnumValue error");
        }
    }

    #[test]
    fn deserialize_optional_address_returns_none_for_zero() {
        #[derive(Deserialize)]
        struct TestStruct {
            #[serde(deserialize_with = "deserialize_optional_address")]
            address: Option<Address>,
        }

        let json = r#"{"address": "0"}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();

        assert!(result.address.is_none());
    }

    #[test]
    fn deserialize_optional_address_returns_none_for_empty() {
        #[derive(Deserialize)]
        struct TestStruct {
            #[serde(deserialize_with = "deserialize_optional_address")]
            address: Option<Address>,
        }

        let json = r#"{"address": ""}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();

        assert!(result.address.is_none());
    }

    #[test]
    fn deserialize_optional_address_returns_some_for_valid() {
        #[derive(Deserialize)]
        struct TestStruct {
            #[serde(deserialize_with = "deserialize_optional_address")]
            address: Option<Address>,
        }

        let json = r#"{"address": "0xdeadbeef"}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();

        assert!(result.address.is_some());
        assert_eq!(result.address.unwrap().as_str(), "deadbeef");
    }

    #[test]
    fn deserialize_optional_string_returns_none_for_empty() {
        #[derive(Deserialize)]
        struct TestStruct {
            #[serde(deserialize_with = "deserialize_optional_string")]
            value: Option<String>,
        }

        let json = r#"{"value": ""}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();

        assert!(result.value.is_none());
    }

    #[test]
    fn deserialize_optional_string_returns_some_for_non_empty() {
        #[derive(Deserialize)]
        struct TestStruct {
            #[serde(deserialize_with = "deserialize_optional_string")]
            value: Option<String>,
        }

        let json = r#"{"value": "test"}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();

        assert!(result.value.is_some());
        assert_eq!(result.value.unwrap(), "test");
    }
}
