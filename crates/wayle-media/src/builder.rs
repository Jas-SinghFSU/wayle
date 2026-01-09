//! Builder for configuring a MediaService.

use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::info;
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;
use zbus::Connection;

use crate::{error::Error, service::MediaService};

/// Builder for configuring and creating a MediaService instance.
///
/// Allows customization of ignored player patterns for filtering out
/// specific media players from being tracked.
#[derive(Default)]
pub struct MediaServiceBuilder {
    ignored_players: Vec<String>,
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

    /// Builds and initializes the MediaService.
    ///
    /// This will establish a D-Bus session connection and start monitoring
    /// for media player changes.
    ///
    /// # Errors
    /// Returns error if D-Bus connection fails or monitoring cannot be started.
    pub async fn build(self) -> Result<MediaService, Error> {
        info!("Starting MPRIS service with property-based architecture");

        let connection = Connection::session()
            .await
            .map_err(|e| Error::InitializationFailed(format!("D-Bus connection failed: {e}")))?;

        let cancellation_token = CancellationToken::new();

        let service = MediaService {
            connection,
            players: Arc::new(RwLock::new(HashMap::new())),
            player_list: Property::new(Vec::new()),
            active_player: Property::new(None),
            ignored_patterns: self.ignored_players,
            cancellation_token: cancellation_token.clone(),
        };

        service.start_monitoring().await?;

        Ok(service)
    }
}
