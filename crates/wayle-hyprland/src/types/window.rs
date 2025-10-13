use serde::{Deserialize, Deserializer};

/// Window dimensions in pixels.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct WindowSize {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

pub(crate) fn deserialize_window_size<'de, D>(deserializer: D) -> Result<WindowSize, D::Error>
where
    D: Deserializer<'de>,
{
    let [width, height]: [u32; 2] = Deserialize::deserialize(deserializer)?;

    Ok(WindowSize { width, height })
}

/// Window position in screen coordinates.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct WindowLocation {
    /// X coordinate in pixels.
    pub x: i32,
    /// Y coordinate in pixels.
    pub y: i32,
}

pub(crate) fn deserialize_window_location<'de, D>(
    deserializer: D,
) -> Result<WindowLocation, D::Error>
where
    D: Deserializer<'de>,
{
    let [x, y]: [i32; 2] = Deserialize::deserialize(deserializer)?;

    Ok(WindowLocation { x, y })
}

/// Window fullscreen state.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(from = "u8")]
pub enum FullscreenMode {
    /// Not fullscreen.
    None = 0,
    /// Fullscreen mode.
    Full = 1,
    /// Maximized mode.
    Maximize = 2,
}

impl From<u8> for FullscreenMode {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Full,
            2 => Self::Maximize,
            _ => Self::None,
        }
    }
}
