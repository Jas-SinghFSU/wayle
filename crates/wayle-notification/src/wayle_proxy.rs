//! D-Bus client proxy for Wayle notification extensions.
#![allow(missing_docs)]

use zbus::{Result, proxy};

/// D-Bus client proxy for Wayle notification extensions.
///
/// Connects to a running notification daemon and allows external control
/// of notifications, DND mode, and popup settings.
#[proxy(
    interface = "com.wayle.Notifications1",
    default_service = "com.wayle.Notifications1",
    default_path = "/com/wayle/Notifications",
    gen_blocking = false
)]
pub trait WayleNotifications {
    /// Dismisses all notifications.
    async fn dismiss_all(&self) -> Result<()>;

    /// Dismisses a specific notification by ID.
    async fn dismiss(&self, id: u32) -> Result<()>;

    /// Sets Do Not Disturb mode.
    async fn set_dnd(&self, enabled: bool) -> Result<()>;

    /// Toggles Do Not Disturb mode.
    async fn toggle_dnd(&self) -> Result<()>;

    /// Sets the popup display duration in milliseconds.
    async fn set_popup_duration(&self, duration_ms: u32) -> Result<()>;

    /// Lists all notifications.
    ///
    /// Returns a list of tuples: (id, app_name, summary, body).
    async fn list(&self) -> Result<Vec<(u32, String, String, String)>>;

    /// Do Not Disturb status.
    #[zbus(property)]
    fn dnd(&self) -> Result<bool>;

    /// Popup display duration in milliseconds.
    #[zbus(property)]
    fn popup_duration(&self) -> Result<u32>;

    /// Number of notifications.
    #[zbus(property)]
    fn count(&self) -> Result<u32>;

    /// Number of active popups.
    #[zbus(property)]
    fn popup_count(&self) -> Result<u32>;
}
