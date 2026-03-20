use serde::{Deserialize, Deserializer};

use crate::{
    Address, FocusHistoryId, MonitorId, ProcessId, WorkspaceInfo, deserialize_optional_address,
    deserialize_optional_string,
};

/// Window dimensions in pixels.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ClientSize {
    /// Width in pixels.
    ///
    /// Hyprland may transiently report negative values during resize/animation churn.
    pub width: i32,
    /// Height in pixels.
    ///
    /// Hyprland may transiently report negative values during resize/animation churn.
    pub height: i32,
}

pub(crate) fn deserialize_window_size<'de, D>(deserializer: D) -> Result<ClientSize, D::Error>
where
    D: Deserializer<'de>,
{
    let [width, height]: [i32; 2] = Deserialize::deserialize(deserializer)?;

    Ok(ClientSize { width, height })
}

/// Window position in screen coordinates.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ClientLocation {
    /// X coordinate in pixels.
    pub x: i32,
    /// Y coordinate in pixels.
    pub y: i32,
}

pub(crate) fn deserialize_window_location<'de, D>(
    deserializer: D,
) -> Result<ClientLocation, D::Error>
where
    D: Deserializer<'de>,
{
    let [x, y]: [i32; 2] = Deserialize::deserialize(deserializer)?;

    Ok(ClientLocation { x, y })
}

/// Window fullscreen state matching Hyprland's `eFullscreenMode`.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(from = "u8")]
pub enum FullscreenMode {
    /// Not fullscreen.
    None = 0,
    /// Maximized.
    Maximized = 1,
    /// Fullscreen.
    Fullscreen = 2,
    /// Both maximized and fullscreen.
    MaximizedFullscreen = 3,
}

impl From<u8> for FullscreenMode {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Maximized,
            2 => Self::Fullscreen,
            3 => Self::MaximizedFullscreen,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClientData {
    pub address: Address,
    pub mapped: bool,
    pub hidden: bool,
    #[serde(deserialize_with = "deserialize_window_location")]
    pub at: ClientLocation,
    #[serde(deserialize_with = "deserialize_window_size")]
    pub size: ClientSize,
    pub workspace: WorkspaceInfo,
    pub floating: bool,
    pub monitor: MonitorId,
    pub class: String,
    pub title: String,
    pub initial_class: String,
    pub initial_title: String,
    pub pid: ProcessId,
    pub xwayland: bool,
    pub pinned: bool,
    pub fullscreen: FullscreenMode,
    pub fullscreen_client: FullscreenMode,
    pub over_fullscreen: bool,
    pub grouped: Vec<Address>,
    pub tags: Vec<String>,
    #[serde(deserialize_with = "deserialize_optional_address")]
    pub swallowing: Option<Address>,
    #[serde(rename = "focusHistoryID")]
    pub focus_history_id: FocusHistoryId,
    pub inhibiting_idle: bool,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub xdg_tag: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub xdg_description: Option<String>,
    pub content_type: String,
    pub stable_id: String,
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[test]
    fn fullscreen_mode_from_u8_converts_maximized() {
        let mode = FullscreenMode::from(1u8);

        assert_eq!(mode, FullscreenMode::Maximized);
    }

    #[test]
    fn fullscreen_mode_from_u8_converts_fullscreen() {
        let mode = FullscreenMode::from(2u8);

        assert_eq!(mode, FullscreenMode::Fullscreen);
    }

    #[test]
    fn fullscreen_mode_from_u8_converts_combined() {
        let mode = FullscreenMode::from(3u8);

        assert_eq!(mode, FullscreenMode::MaximizedFullscreen);
    }

    #[test]
    fn fullscreen_mode_from_u8_defaults_to_none() {
        assert_eq!(FullscreenMode::from(0u8), FullscreenMode::None);
        assert_eq!(FullscreenMode::from(99u8), FullscreenMode::None);
    }

    #[test]
    fn deserialize_window_size_creates_correct_struct() {
        #[derive(Deserialize)]
        struct TestStruct {
            #[serde(deserialize_with = "deserialize_window_size")]
            size: ClientSize,
        }

        let json = r#"{"size": [1920, 1080]}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();

        assert_eq!(result.size.width, 1920);
        assert_eq!(result.size.height, 1080);
    }

    #[test]
    fn deserialize_window_location_creates_correct_struct() {
        #[derive(Deserialize)]
        struct TestStruct {
            #[serde(deserialize_with = "deserialize_window_location")]
            location: ClientLocation,
        }

        let json = r#"{"location": [100, 200]}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();

        assert_eq!(result.location.x, 100);
        assert_eq!(result.location.y, 200);
    }

    #[test]
    fn deserialize_window_size_accepts_negative_values() {
        #[derive(Deserialize)]
        struct TestStruct {
            #[serde(deserialize_with = "deserialize_window_size")]
            size: ClientSize,
        }

        let json = r#"{"size": [-3, -1]}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();

        assert_eq!(result.size.width, -3);
        assert_eq!(result.size.height, -1);
    }
}
