use serde::Deserialize;

/// Keybind configuration from Hyprland.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BindData {
    /// Bind works even when the screen is locked.
    pub locked: bool,
    /// Mouse button bind.
    pub mouse: bool,
    /// Triggers on key release instead of press.
    pub release: bool,
    /// Repeats while key is held down.
    pub repeat: bool,
    /// Triggers after a long press.
    pub long_press: bool,
    /// Non-consuming bind allows other binds to trigger.
    #[serde(alias = "non_consuming")]
    pub non_consuming: bool,
    /// Bind has a description.
    #[serde(alias = "has_description")]
    pub has_description: bool,
    /// Bitmask of modifier keys.
    pub modmask: u32,
    /// Submap this bind belongs to.
    pub submap: String,
    /// Bind is active in all submaps.
    #[serde(alias = "submap_universal")]
    pub submap_universal: String,
    /// Key name.
    pub key: String,
    /// Numerical keycode.
    pub keycode: i32,
    /// Catches all unmatched keys.
    #[serde(alias = "catch_all")]
    pub catch_all: bool,
    /// Human-readable description.
    pub description: String,
    /// Dispatcher command to execute.
    pub dispatcher: String,
    /// Arguments for the dispatcher.
    pub arg: String,
}
