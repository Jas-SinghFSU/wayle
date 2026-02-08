use std::{collections::HashMap, sync::Arc};

use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::info;
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;
use zbus::Connection;

use crate::{
    backend::{TransitionConfig, spawn_daemon_if_needed},
    dbus::{SERVICE_NAME, SERVICE_PATH, WallpaperDaemon},
    error::Error,
    service::WallpaperService,
    tasks::{spawn_color_extractor, spawn_output_watcher},
    types::{ColorExtractor, FitMode},
};

/// Builder for configuring a WallpaperService.
#[derive(Debug, Default)]
pub struct WallpaperServiceBuilder {
    fit_mode: FitMode,
    color_extractor: ColorExtractor,
    transition: TransitionConfig,
}

impl WallpaperServiceBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the default fit mode.
    pub fn fit_mode(mut self, mode: FitMode) -> Self {
        self.fit_mode = mode;
        self
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

    /// Builds and initializes the WallpaperService.
    ///
    /// # Errors
    ///
    /// Returns error if D-Bus connection fails or service registration fails.
    pub async fn build(self) -> Result<Arc<WallpaperService>, Error> {
        spawn_daemon_if_needed();

        let connection = Connection::session().await.map_err(|err| {
            Error::ServiceInitializationFailed(format!("D-Bus connection failed: {err}"))
        })?;

        let cancellation_token = CancellationToken::new();
        let (extraction_complete, _) = broadcast::channel(16);

        let service = Arc::new(WallpaperService {
            cancellation_token,
            _connection: connection.clone(),
            last_extracted_wallpaper: Property::new(None),
            extraction_complete,
            fit_mode: Property::new(self.fit_mode),
            theming_monitor: Property::new(None),
            cycling: Property::new(None),
            monitors: Property::new(HashMap::new()),
            color_extractor: Property::new(self.color_extractor),
            transition: Property::new(self.transition),
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

        info!("Wallpaper service registered at {SERVICE_NAME}");

        service.start_monitoring().await?;
        spawn_output_watcher(Arc::clone(&service));
        spawn_color_extractor(Arc::clone(&service));

        Ok(service)
    }
}
