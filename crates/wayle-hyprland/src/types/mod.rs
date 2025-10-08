mod layer;
mod monitor;
mod window;
mod workspace;

use std::fmt::{self, Display};

pub use layer::*;
pub use monitor::*;
use serde::{Deserialize, Deserializer};
pub use window::*;

pub type MonitorId = i64;
pub type WorkspaceId = i64;
pub type ProcessId = i32;
pub type FocusHistoryId = i32;

/// The type of screencopy share.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreencastOwner {
    /// Monitor share.
    Monitor,
    /// Window share.
    Window,
}

/// Workspace information for a monitor.
#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct WorkspaceInfo {
    /// Workspace ID.
    pub id: WorkspaceId,
    /// Workspace name.
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(transparent)]
pub struct Address(String);

impl Address {
    pub fn new(address: String) -> Self {
        Self(address)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

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
