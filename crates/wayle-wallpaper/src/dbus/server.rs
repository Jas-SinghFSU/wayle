//! D-Bus server interface implementation.

use std::{path::PathBuf, sync::Arc, time::Duration};

use tracing::instrument;
use zbus::fdo;

use crate::{service::WallpaperService, types::CyclingMode};

/// D-Bus daemon for external control of the wallpaper service.
#[derive(Debug)]
pub(crate) struct WallpaperDaemon {
    pub service: Arc<WallpaperService>,
}

impl WallpaperDaemon {
    fn monitor_option(monitor: &str) -> Option<&str> {
        if monitor.is_empty() {
            None
        } else {
            Some(monitor)
        }
    }
}

#[zbus::interface(name = "com.wayle.Wallpaper1")]
impl WallpaperDaemon {
    /// Sets the wallpaper for a specific monitor or all monitors.
    ///
    /// Pass an empty string for `monitor` to target all monitors.
    #[instrument(skip(self), fields(path = %path, monitor = %monitor))]
    pub async fn set_wallpaper(&self, path: String, monitor: String) -> fdo::Result<()> {
        self.service
            .set_wallpaper(PathBuf::from(&path), Self::monitor_option(&monitor))
            .await
            .map_err(|err| fdo::Error::Failed(err.to_string()))
    }

    /// Sets the fit mode for wallpaper scaling.
    #[instrument(skip(self), fields(mode = %mode))]
    pub async fn set_fit_mode(&self, mode: String) -> fdo::Result<()> {
        let fit_mode = mode
            .parse()
            .map_err(|err: String| fdo::Error::InvalidArgs(err))?;

        self.service
            .set_fit_mode(fit_mode)
            .await
            .map_err(|err| fdo::Error::Failed(err.to_string()))
    }

    /// Starts cycling through wallpapers in a directory.
    ///
    /// All monitors cycle from the same directory with the same interval.
    /// Each monitor shows a different image from the pool.
    #[instrument(skip(self), fields(dir = %directory, interval = %interval_secs, mode = %mode))]
    pub async fn start_cycling(
        &self,
        directory: String,
        interval_secs: u32,
        mode: String,
    ) -> fdo::Result<()> {
        let cycling_mode: CyclingMode = mode
            .parse()
            .map_err(|err: String| fdo::Error::InvalidArgs(err))?;

        self.service
            .start_cycling(
                PathBuf::from(directory),
                Duration::from_secs(u64::from(interval_secs)),
                cycling_mode,
            )
            .await
            .map_err(|err| fdo::Error::Failed(err.to_string()))
    }

    /// Stops wallpaper cycling.
    #[instrument(skip(self))]
    pub async fn stop_cycling(&self) -> fdo::Result<()> {
        self.service.stop_cycling();
        Ok(())
    }

    /// Advances all monitors to the next wallpaper.
    #[instrument(skip(self))]
    pub async fn next(&self) -> fdo::Result<()> {
        self.service
            .advance_cycle()
            .await
            .map_err(|err| fdo::Error::Failed(err.to_string()))
    }

    /// Goes back to the previous wallpaper for all monitors.
    #[instrument(skip(self))]
    pub async fn previous(&self) -> fdo::Result<()> {
        self.service
            .rewind_cycle()
            .await
            .map_err(|err| fdo::Error::Failed(err.to_string()))
    }

    /// Extracts colors from the current wallpaper.
    #[instrument(skip(self))]
    pub async fn extract_colors(&self) -> fdo::Result<()> {
        self.service
            .extract_colors()
            .await
            .map_err(|err| fdo::Error::Failed(err.to_string()))
    }

    /// Gets the wallpaper path for a specific monitor.
    #[instrument(skip(self), fields(monitor = %monitor))]
    pub async fn wallpaper_for_monitor(&self, monitor: String) -> String {
        self.service
            .wallpaper(&monitor)
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    /// Gets the current fit mode.
    #[instrument(skip(self))]
    pub async fn get_fit_mode(&self) -> String {
        self.service.fit_mode.get().to_string()
    }

    /// Checks if cycling is active.
    #[instrument(skip(self))]
    pub async fn get_is_cycling(&self) -> bool {
        self.service.cycling_config().is_some()
    }

    /// Sets which monitor to use for color extraction theming.
    #[instrument(skip(self), fields(monitor = %monitor))]
    pub async fn set_theming_monitor(&self, monitor: String) {
        self.service
            .set_theming_monitor(Self::monitor_option(&monitor).map(String::from));
    }

    /// Registers a monitor for wallpaper management.
    #[instrument(skip(self), fields(monitor = %monitor))]
    pub async fn register_monitor(&self, monitor: String) {
        self.service.register_monitor(&monitor);
    }

    /// Unregisters a monitor.
    #[instrument(skip(self), fields(monitor = %monitor))]
    pub async fn unregister_monitor(&self, monitor: String) {
        self.service.unregister_monitor(&monitor);
    }

    /// Lists all registered monitors.
    #[instrument(skip(self))]
    pub async fn list_monitors(&self) -> Vec<String> {
        self.service.monitor_names()
    }

    /// Current fit mode.
    #[zbus(property)]
    pub async fn fit_mode(&self) -> String {
        self.service.fit_mode.get().to_string()
    }

    /// Whether cycling is active.
    #[zbus(property)]
    pub async fn is_cycling(&self) -> bool {
        self.service.cycling_config().is_some()
    }

    /// The monitor used for color extraction theming (empty = default).
    #[zbus(property)]
    pub async fn theming_monitor(&self) -> String {
        self.service.theming_monitor.get().unwrap_or_default()
    }
}
