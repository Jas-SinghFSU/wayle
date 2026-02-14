use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Duration};

use derive_more::Debug;
use futures::{
    future::try_join_all,
    stream::{Stream, StreamExt},
};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_util::sync::CancellationToken;
use tracing::{info, instrument, warn};
use wayle_common::Property;
use zbus::Connection;

use crate::{
    backend::{SwwwBackend, TransitionConfig},
    builder::WallpaperServiceBuilder,
    error::Error,
    types::{ColorExtractor, CyclingConfig, CyclingMode, FitMode, MonitorState},
};

/// Desktop wallpaper manager. See [crate-level docs](crate) for usage.
#[derive(Debug)]
pub struct WallpaperService {
    #[debug(skip)]
    pub(crate) cancellation_token: CancellationToken,
    #[debug(skip)]
    pub(crate) _connection: Connection,
    #[debug(skip)]
    pub(crate) last_extracted_wallpaper: Property<Option<PathBuf>>,
    #[debug(skip)]
    pub(crate) extraction_complete: broadcast::Sender<()>,

    /// Monitor used for color extraction, or first available if `None`.
    pub theming_monitor: Property<Option<String>>,
    /// Active cycling state, or `None` when cycling is stopped.
    pub cycling: Property<Option<CyclingConfig>>,
    /// Per-monitor wallpaper state (path, fit mode, cycle index).
    pub monitors: Property<HashMap<String, MonitorState>>,
    /// Tool for extracting color palettes from wallpapers.
    pub color_extractor: Property<ColorExtractor>,
    /// Animation settings for wallpaper transitions.
    pub transition: Property<TransitionConfig>,
    /// Synchronize cycling across all monitors in shuffle mode.
    ///
    /// When `true`, all monitors show the same image during shuffle cycling.
    /// Sequential mode always shows the same image regardless of this setting.
    pub shared_cycle: Property<bool>,
    /// Whether wayle's built-in wallpaper engine (swww) is active.
    ///
    /// When `false`, all state tracking and color extraction continue but
    /// swww commands are skipped.
    pub engine_active: Property<bool>,
}

impl WallpaperService {
    /// Creates a new wallpaper service with default configuration.
    ///
    /// # Errors
    ///
    /// Returns error if D-Bus connection fails or service registration fails.
    #[instrument(name = "WallpaperService::new", err)]
    pub async fn new() -> Result<Arc<Self>, Error> {
        Self::builder().build().await
    }

    /// Creates a builder for configuring the wallpaper service.
    pub fn builder() -> WallpaperServiceBuilder {
        WallpaperServiceBuilder::new()
    }

    /// Returns the wallpaper for a monitor.
    ///
    /// Returns `None` if the monitor isn't registered or has no wallpaper.
    pub fn wallpaper(&self, monitor: &str) -> Option<PathBuf> {
        self.monitors
            .get()
            .get(monitor)
            .and_then(|state| state.wallpaper.clone())
    }

    /// Returns the cycling configuration, if cycling is active.
    pub fn cycling_config(&self) -> Option<CyclingConfig> {
        self.cycling.get()
    }

    /// Sets a wallpaper on a specific monitor or all monitors.
    ///
    /// Uses each monitor's own fit mode for scaling.
    /// If `monitor` is `None`, applies to all known monitors.
    ///
    /// # Errors
    ///
    /// Returns error if the image file does not exist, swww is not installed,
    /// or the swww daemon is not running.
    #[instrument(skip(self), fields(path = %path.display(), monitor))]
    pub async fn set_wallpaper(&self, path: PathBuf, monitor: Option<&str>) -> Result<(), Error> {
        if !path.exists() {
            return Err(Error::ImageNotFound(path));
        }

        match monitor {
            Some(name) => {
                self.store_wallpaper(name, path.clone());

                if self.engine_active.get() {
                    let fit_mode = self
                        .monitors
                        .get()
                        .get(name)
                        .map(|s| s.fit_mode)
                        .unwrap_or_default();
                    let transition = self.transition.get();
                    SwwwBackend::apply(&path, fit_mode, Some(name), &transition).await?;
                }
            }
            None => {
                self.store_wallpaper_all(path.clone());
                self.rerender_all().await?;
            }
        }

        Ok(())
    }

    /// Returns the fit mode for a monitor.
    ///
    /// Returns `None` if the monitor isn't registered.
    pub fn fit_mode(&self, monitor: &str) -> Option<FitMode> {
        self.monitors.get().get(monitor).map(|s| s.fit_mode)
    }

    /// Sets the fit mode for a monitor and re-applies its wallpaper.
    ///
    /// If `monitor` is `None`, sets the fit mode for all monitors.
    ///
    /// # Errors
    ///
    /// Returns error if swww fails to apply wallpapers.
    #[instrument(skip(self), fields(mode = %mode, monitor))]
    pub async fn set_fit_mode(&self, mode: FitMode, monitor: Option<&str>) -> Result<(), Error> {
        let mut monitors = self.monitors.get();

        match monitor {
            Some(name) => {
                let Some(state) = monitors.get_mut(name) else {
                    return Ok(());
                };
                state.fit_mode = mode;
                let path = state.wallpaper.clone();
                self.monitors.set(monitors);

                if self.engine_active.get()
                    && let Some(path) = path
                {
                    let transition = self.transition.get();
                    SwwwBackend::apply(&path, mode, Some(name), &transition).await?;
                }
            }
            None => {
                for state in monitors.values_mut() {
                    state.fit_mode = mode;
                }
                self.monitors.set(monitors);
                self.rerender_all().await?;
            }
        }

        Ok(())
    }

    /// Starts cycling through wallpapers in a directory.
    ///
    /// All monitors cycle from the same directory with the same interval.
    /// Each monitor gets a different starting index so they show different images.
    /// The first wallpaper is immediately applied to each monitor.
    ///
    /// # Errors
    ///
    /// Returns error if the directory doesn't exist, contains no valid images,
    /// or swww fails to apply wallpapers.
    #[instrument(skip(self), fields(dir = %directory.display()))]
    pub fn start_cycling(
        &self,
        directory: PathBuf,
        interval: Duration,
        mode: CyclingMode,
    ) -> Result<(), Error> {
        let config = CyclingConfig::new(directory, mode, interval)?;
        let image_count = config.image_count();

        self.reset_cycle_indices(mode, image_count);
        self.cycling.set(Some(config));
        Ok(())
    }

    /// Stops wallpaper cycling.
    #[instrument(skip(self))]
    pub fn stop_cycling(&self) {
        self.cycling.set(None);
    }

    /// Sets the cycling interval.
    ///
    /// Takes effect immediately if cycling is active.
    #[instrument(skip(self), fields(interval_secs = interval.as_secs()))]
    pub fn set_cycling_interval(&self, interval: Duration) {
        let mut cycling = self.cycling.get();
        if let Some(ref mut config) = cycling {
            config.interval = interval;
            self.cycling.set(cycling);
        }
    }

    /// Advances all monitors to their next wallpaper in the cycle.
    ///
    /// Each monitor advances its own index in the shared image pool.
    ///
    /// # Errors
    ///
    /// Returns error if swww fails to apply wallpapers.
    #[instrument(skip(self))]
    pub async fn advance_cycle(&self) -> Result<(), Error> {
        let Some(config) = self.cycling.get() else {
            return Ok(());
        };

        let image_count = config.image_count();
        if image_count == 0 {
            return Ok(());
        }

        let mut monitors = self.monitors.get();
        for state in monitors.values_mut() {
            state.advance(image_count);
        }
        self.monitors.set(monitors);

        self.render_cycle().await
    }

    /// Rewinds all monitors to their previous wallpaper in the cycle.
    ///
    /// Each monitor goes back to its previous index.
    ///
    /// # Errors
    ///
    /// Returns error if swww fails to apply wallpapers.
    #[instrument(skip(self))]
    pub async fn rewind_cycle(&self) -> Result<(), Error> {
        let Some(config) = self.cycling.get() else {
            return Ok(());
        };

        let image_count = config.image_count();
        if image_count == 0 {
            return Ok(());
        }

        let mut monitors = self.monitors.get();
        for state in monitors.values_mut() {
            state.previous(image_count);
        }
        self.monitors.set(monitors);

        self.render_cycle().await
    }

    /// Extracts colors from the theming monitor's wallpaper.
    ///
    /// Uses wallpaper from `theming_monitor` if configured. Falls back to
    /// first monitor otherwise.
    ///
    /// # Errors
    ///
    /// Returns error if no wallpaper is set or color extraction fails.
    #[instrument(skip(self))]
    pub(crate) async fn extract_colors(&self) -> Result<(), Error> {
        let monitors = self.monitors.get();

        let path = self
            .theming_monitor
            .get()
            .and_then(|monitor| monitors.get(&monitor))
            .or_else(|| monitors.values().next())
            .and_then(|state| state.wallpaper.clone());

        if self.last_extracted_wallpaper.get() == path {
            let _ = self.extraction_complete.send(());
            return Ok(());
        };

        self.last_extracted_wallpaper.set(path.clone());

        let Some(path) = path else {
            let _ = self.extraction_complete.send(());
            return Ok(());
        };

        let extractor = self.color_extractor.get();
        let result = extractor.extract(&path).await;

        let _ = self.extraction_complete.send(());

        result
    }

    /// Sets which monitor's wallpaper to use for color extraction.
    ///
    /// Pass `None` to use the first available monitor.
    #[instrument(skip(self), fields(monitor))]
    pub fn set_theming_monitor(&self, monitor: Option<String>) {
        self.theming_monitor.set(monitor);
    }

    /// Returns all known monitor names.
    pub fn monitor_names(&self) -> Vec<String> {
        self.monitors.get().keys().cloned().collect()
    }

    /// Returns a stream that emits when color extraction completes.
    ///
    /// Subscribers can listen for palette changes from matugen, wallust, or pywal.
    pub fn watch_extraction(&self) -> impl Stream<Item = ()> + Send + 'static {
        BroadcastStream::new(self.extraction_complete.subscribe())
            .filter_map(|result| async { result.ok() })
    }

    /// Registers a monitor.
    ///
    /// New monitors start with no wallpaper and a unique cycle index
    /// (distributed evenly across the image pool if cycling is active).
    pub fn register_monitor(&self, monitor: &str) {
        let mut monitors = self.monitors.get();
        if monitors.contains_key(monitor) {
            return;
        }

        let cycle_index = self.new_monitor_starting_index();
        monitors.insert(
            monitor.to_string(),
            MonitorState::with_cycle_index(cycle_index),
        );
        self.monitors.set(monitors);

        info!(monitor, cycle_index, "Monitor registered");
    }

    /// Unregisters a monitor.
    pub fn unregister_monitor(&self, monitor: &str) {
        let mut monitors = self.monitors.get();
        if monitors.remove(monitor).is_some() {
            self.monitors.set(monitors);
            info!(monitor, "Monitor unregistered");
        }
    }

    /// Sets the transition animation configuration.
    #[instrument(skip(self))]
    pub fn set_transition(&self, transition: TransitionConfig) {
        self.transition.set(transition);
    }

    /// Renders the current cycle wallpaper to each monitor.
    async fn render_cycle(&self) -> Result<(), Error> {
        let Some(config) = self.cycling.get() else {
            return Ok(());
        };

        let mut monitors = self.monitors.get();
        let mut to_apply = Vec::new();

        for (monitor_name, state) in monitors.iter_mut() {
            let Some(path) = config.image_at(state.cycle_index) else {
                continue;
            };
            state.wallpaper = Some(path.clone());
            to_apply.push((monitor_name.clone(), path, state.fit_mode));
        }

        self.monitors.set(monitors);

        if self.engine_active.get() {
            let transition = self.transition.get();
            let futures = to_apply.iter().map(|(name, path, fit_mode)| {
                SwwwBackend::apply(path, *fit_mode, Some(name.as_str()), &transition)
            });
            try_join_all(futures).await?;
        }

        Ok(())
    }

    /// Renders all monitors' wallpapers in a background task.
    ///
    /// State must already be updated via `monitors.set()` before calling this.
    /// Only the swww rendering runs in the background.
    pub fn render_all_background(self: &Arc<Self>) {
        let service = Arc::clone(self);
        tokio::spawn(async move {
            if let Err(e) = service.rerender_all().await {
                warn!(error = %e, "background wallpaper render failed");
            }
        });
    }

    /// Re-renders all monitors with their current wallpaper.
    async fn rerender_all(&self) -> Result<(), Error> {
        if !self.engine_active.get() {
            return Ok(());
        }

        let monitors = self.monitors.get();
        let transition = self.transition.get();

        let futures = monitors.iter().filter_map(|(name, state)| {
            state.wallpaper.as_ref().map(|path| {
                SwwwBackend::apply(path, state.fit_mode, Some(name.as_str()), &transition)
            })
        });
        try_join_all(futures).await?;

        Ok(())
    }

    fn store_wallpaper(&self, monitor: &str, path: PathBuf) {
        let mut monitors = self.monitors.get();
        if let Some(state) = monitors.get_mut(monitor) {
            state.wallpaper = Some(path);
            self.monitors.set(monitors);
        }
    }

    fn store_wallpaper_all(&self, path: PathBuf) {
        let mut monitors = self.monitors.get();
        for state in monitors.values_mut() {
            state.wallpaper = Some(path.clone());
        }
        self.monitors.set(monitors);
    }

    fn reset_cycle_indices(&self, mode: CyclingMode, image_count: usize) {
        if image_count == 0 {
            return;
        }

        let shared = self.shared_cycle.get();
        let shared_index = rand::random_range(0..image_count);
        let mut monitors = self.monitors.get();

        for state in monitors.values_mut() {
            state.cycle_index = match mode {
                CyclingMode::Sequential => 0,
                CyclingMode::Shuffle if shared => shared_index,
                CyclingMode::Shuffle => rand::random_range(0..image_count),
            };
        }

        self.monitors.set(monitors);
    }

    fn new_monitor_starting_index(&self) -> usize {
        let Some(config) = self.cycling.get() else {
            return 0;
        };

        let image_count = config.image_count();
        if image_count == 0 {
            return 0;
        }

        match config.mode {
            CyclingMode::Sequential => 0,
            CyclingMode::Shuffle if self.shared_cycle.get() => self
                .monitors
                .get()
                .values()
                .next()
                .map(|s| s.cycle_index)
                .unwrap_or(0),
            CyclingMode::Shuffle => rand::random_range(0..image_count),
        }
    }
}

impl Drop for WallpaperService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
