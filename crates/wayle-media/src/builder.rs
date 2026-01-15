//! Builder for configuring a MediaService.

use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::info;
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;
use zbus::Connection;

use crate::{
    dbus::{MediaDaemon, SERVICE_NAME, SERVICE_PATH},
    error::Error,
    service::MediaService,
};

/// Builder for configuring and creating a MediaService instance.
///
/// Allows customization of ignored player patterns for filtering out
/// specific media players from being tracked.
#[derive(Default)]
pub struct MediaServiceBuilder {
    ignored_players: Vec<String>,
    register_daemon: bool,
}

impl MediaServiceBuilder {
    /// Creates a new MediaServiceBuilder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the patterns for media players to ignore.
    ///
    /// Players whose names match these patterns will not be tracked by the service.
    pub fn ignored_players(mut self, patterns: Vec<String>) -> Self {
        self.ignored_players = patterns;
        self
    }

    /// Adds a single pattern for a media player to ignore.
    pub fn ignore_player(mut self, pattern: String) -> Self {
        self.ignored_players.push(pattern);
        self
    }

    /// Enables D-Bus daemon registration for external control.
    ///
    /// When enabled, the service will register itself on the session bus
    /// at `com.wayle.Media1`, allowing CLI tools and other applications
    /// to control media playback.
    pub fn with_daemon(mut self) -> Self {
        self.register_daemon = true;
        self
    }

    /// Builds and initializes the MediaService.
    ///
    /// This will establish a D-Bus session connection and start monitoring
    /// for media player changes. If `with_daemon()` was called, the service
    /// will also register on the session bus for external control.
    ///
    /// # Errors
    /// Returns error if D-Bus connection fails or monitoring cannot be started.
    pub async fn build(self) -> Result<Arc<MediaService>, Error> {
        info!("Starting MPRIS service with property-based architecture");

        let connection = Connection::session()
            .await
            .map_err(|e| Error::Initialization(format!("d-bus connection: {e}")))?;

        let cancellation_token = CancellationToken::new();

        let service = Arc::new(MediaService {
            connection: connection.clone(),
            players: Arc::new(RwLock::new(HashMap::new())),
            player_list: Property::new(Vec::new()),
            active_player: Property::new(None),
            ignored_patterns: self.ignored_players,
            cancellation_token: cancellation_token.clone(),
        });

        service.start_monitoring().await?;

        if self.register_daemon {
            let daemon = MediaDaemon {
                service: Arc::clone(&service),
            };

            connection
                .object_server()
                .at(SERVICE_PATH, daemon)
                .await
                .map_err(|e| {
                    Error::Initialization(format!(
                        "cannot register d-bus object at '{SERVICE_PATH}': {e}"
                    ))
                })?;

            connection.request_name(SERVICE_NAME).await.map_err(|e| {
                Error::Initialization(format!("cannot acquire d-bus name '{SERVICE_NAME}': {e}"))
            })?;

            info!("Media service registered at {SERVICE_NAME}");
        }

        Ok(service)
    }
}
