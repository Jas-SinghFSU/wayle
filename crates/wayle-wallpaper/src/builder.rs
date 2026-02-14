use std::{collections::HashMap, sync::Arc, time::Instant};

use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info};
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;
use zbus::Connection;

use crate::{
    backend::{TransitionConfig, spawn_daemon_if_needed},
    dbus::{SERVICE_NAME, SERVICE_PATH, WallpaperDaemon},
    error::Error,
    service::WallpaperService,
    tasks::{spawn_color_extractor, spawn_output_watcher},
    types::ColorExtractor,
};

/// Builder for configuring a WallpaperService.
#[derive(Debug)]
pub struct WallpaperServiceBuilder {
    color_extractor: ColorExtractor,
    transition: TransitionConfig,
    theming_monitor: Option<String>,
    shared_cycle: bool,
    engine_active: bool,
}

impl Default for WallpaperServiceBuilder {
    fn default() -> Self {
        Self {
            color_extractor: ColorExtractor::default(),
            transition: TransitionConfig::default(),
            theming_monitor: None,
            shared_cycle: false,
            engine_active: true,
        }
    }
}

impl WallpaperServiceBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the color extraction tool.
    pub fn color_extractor(mut self, extractor: ColorExtractor) -> Self {
        self.color_extractor = extractor;
        self
    }

    /// Sets the transition animation configuration.
    pub fn transition(mut self, transition: TransitionConfig) -> Self {
        self.transition = transition;
        self
    }

    /// Sets which monitor's wallpaper drives color extraction.
    pub fn theming_monitor(mut self, monitor: Option<String>) -> Self {
        self.theming_monitor = monitor;
        self
    }

    /// Synchronizes cycling across all monitors in shuffle mode.
    pub fn shared_cycle(mut self, shared: bool) -> Self {
        self.shared_cycle = shared;
        self
    }

    /// Enables or disables the built-in wallpaper engine (swww).
    pub fn engine_active(mut self, active: bool) -> Self {
        self.engine_active = active;
        self
    }

    /// Builds and initializes the WallpaperService.
    ///
    /// # Errors
    ///
    /// Returns error if D-Bus connection fails or service registration fails.
    pub async fn build(self) -> Result<Arc<WallpaperService>, Error> {
        let t = Instant::now();
        if self.engine_active {
            spawn_daemon_if_needed();
        }

        let connection = Connection::session().await.map_err(|err| {
            Error::ServiceInitializationFailed(format!("D-Bus connection failed: {err}"))
        })?;
        debug!(elapsed_ms = t.elapsed().as_millis(), "D-Bus connected");

        let cancellation_token = CancellationToken::new();
        let (extraction_complete, _) = broadcast::channel(16);

        let service = Arc::new(WallpaperService {
            cancellation_token,
            _connection: connection.clone(),
            last_extracted_wallpaper: Property::new(None),
            extraction_complete,
            theming_monitor: Property::new(self.theming_monitor),
            cycling: Property::new(None),
            monitors: Property::new(HashMap::new()),
            color_extractor: Property::new(self.color_extractor),
            transition: Property::new(self.transition),
            shared_cycle: Property::new(self.shared_cycle),
            engine_active: Property::new(self.engine_active),
        });

        let daemon = WallpaperDaemon {
            service: Arc::clone(&service),
        };

        connection
            .object_server()
            .at(SERVICE_PATH, daemon)
            .await
            .map_err(|err| {
                Error::ServiceInitializationFailed(format!(
                    "cannot register D-Bus object at '{SERVICE_PATH}': {err}"
                ))
            })?;

        connection.request_name(SERVICE_NAME).await.map_err(|err| {
            Error::ServiceInitializationFailed(format!(
                "cannot acquire D-Bus name '{SERVICE_NAME}': {err}"
            ))
        })?;

        debug!(elapsed_ms = t.elapsed().as_millis(), "D-Bus registered");

        service.start_monitoring().await?;
        debug!(elapsed_ms = t.elapsed().as_millis(), "Monitoring started");

        spawn_output_watcher(Arc::clone(&service));
        spawn_color_extractor(Arc::clone(&service));

        info!("Wallpaper service registered at {SERVICE_NAME}");

        Ok(service)
    }
}
