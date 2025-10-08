use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

pub(crate) fn deserialize_window_size<'de, D>(deserializer: D) -> Result<WindowSize, D::Error>
where
    D: Deserializer<'de>,
{
    let [width, height]: [u32; 2] = Deserialize::deserialize(deserializer)?;

    Ok(WindowSize { width, height })
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct WindowLocation {
    pub x: i32,
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(from = "u8")]
pub enum FullscreenMode {
    None = 0,
    Full = 1,
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
