//! Builder for configuring a MediaService.

use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::info;
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;
use zbus::Connection;

use crate::{
    core::metadata::art::ArtResolver,
    dbus::{MediaDaemon, SERVICE_NAME, SERVICE_PATH},
    error::Error,
    service::MediaService,
};

/// Builder for configuring and creating a MediaService instance.
///
/// Allows customization of ignored player patterns for filtering out
/// specific media players from being tracked.
pub struct MediaServiceBuilder {
    ignored_players: Vec<String>,
    priority_players: Vec<String>,
    register_daemon: bool,
    enable_art_cache: bool,
    position_poll_interval: Duration,
}

impl Default for MediaServiceBuilder {
    fn default() -> Self {
        Self {
            ignored_players: Vec::new(),
            priority_players: Vec::new(),
            register_daemon: false,
            enable_art_cache: false,
            position_poll_interval: Duration::from_secs(1),
        }
    }
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

    /// Sets the priority order for player selection.
    ///
    /// Patterns are matched against bus names in order. First match wins.
    /// Players matching earlier patterns are preferred when selecting active player.
    pub fn priority_players(mut self, patterns: Vec<String>) -> Self {
        self.priority_players = patterns;
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

    /// Enables album art downloading and disk caching.
    ///
    /// When enabled, HTTP(S) art URLs from MPRIS metadata are downloaded
    /// and cached under `$XDG_CACHE_HOME/wayle/media-art/`. File URLs are
    /// resolved directly. Resolved paths appear on
    /// [`TrackMetadata::cover_art`](crate::core::metadata::TrackMetadata::cover_art).
    pub fn with_art_cache(mut self) -> Self {
        self.enable_art_cache = true;
        self
    }

    /// Sets how often live players refresh the `Position` property.
    ///
    /// This interval is shared across all monitored players and used by a
    /// single polling task per player.
    pub fn position_poll_interval(mut self, interval: Duration) -> Self {
        self.position_poll_interval = interval;
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

        let art_resolver = if self.enable_art_cache {
            ArtResolver::new().await.ok()
        } else {
            None
        };

        let service = Arc::new(MediaService {
            connection: connection.clone(),
            players: Arc::new(RwLock::new(HashMap::new())),
            player_list: Property::new(Vec::new()),
            active_player: Property::new(None),
            ignored_patterns: self.ignored_players,
            priority_patterns: self.priority_players,
            cancellation_token: cancellation_token.clone(),
            art_resolver,
            position_poll_interval: self.position_poll_interval,
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
