//! D-Bus client proxy for the wallpaper service.
#![allow(missing_docs)]

use zbus::{Result, proxy};

/// D-Bus client proxy for controlling the wallpaper service.
///
/// Connects to a running wallpaper daemon and allows external control
/// of wallpaper settings, cycling, and color extraction.
#[proxy(
    interface = "com.wayle.Wallpaper1",
    default_service = "com.wayle.Wallpaper1",
    default_path = "/com/wayle/Wallpaper"
)]
pub trait Wallpaper {
    /// Sets the wallpaper for a specific monitor or all monitors.
    ///
    /// Use an empty string for `monitor` to set on all monitors.
    async fn set_wallpaper(&self, path: String, monitor: String) -> Result<()>;

    /// Sets the fit mode for wallpaper scaling.
    async fn set_fit_mode(&self, mode: String) -> Result<()>;

    /// Starts cycling through wallpapers in a directory.
    async fn start_cycling(
        &self,
        directory: String,
        interval_secs: u32,
        mode: String,
    ) -> Result<()>;

    /// Stops wallpaper cycling.
    async fn stop_cycling(&self) -> Result<()>;

    /// Advances all monitors to the next wallpaper in the cycle.
    async fn next(&self) -> Result<()>;

    /// Rewinds all monitors to the previous wallpaper in the cycle.
    async fn previous(&self) -> Result<()>;

    /// Extracts colors from the current wallpaper.
    async fn extract_colors(&self) -> Result<()>;

    /// Gets the wallpaper path for a specific monitor.
    async fn wallpaper_for_monitor(&self, monitor: String) -> Result<String>;

    /// Gets the current fit mode.
    async fn get_fit_mode(&self) -> Result<String>;

    /// Checks if cycling is active.
    async fn get_is_cycling(&self) -> Result<bool>;

    /// Sets which monitor to use for color extraction theming.
    async fn set_theming_monitor(&self, monitor: String) -> Result<()>;

    /// Registers a monitor for wallpaper management.
    async fn register_monitor(&self, monitor: String) -> Result<()>;

    /// Unregisters a monitor.
    async fn unregister_monitor(&self, monitor: String) -> Result<()>;

    /// Lists all registered monitors.
    async fn list_monitors(&self) -> Result<Vec<String>>;

    /// Current fit mode.
    #[zbus(property)]
    fn fit_mode(&self) -> Result<String>;

    /// Whether cycling is active.
    #[zbus(property)]
    fn is_cycling(&self) -> Result<bool>;

    /// The monitor used for color extraction theming (empty = default).
    #[zbus(property)]
    fn theming_monitor(&self) -> Result<String>;
}
