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
