use std::{collections::HashMap, sync::Arc};

use derive_more::Debug;
use futures::Stream;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::{info, instrument};
use zbus::Connection;

use super::{
    core::player::{LivePlayerParams, Player, PlayerParams},
    types::PlayerId,
};
use crate::services::{
    common::Property,
    media::error::Error,
    traits::{Reactive, ServiceMonitoring},
};

/// MPRIS service with reactive property-based architecture.
///
/// Provides fine-grained reactive updates for efficient UI rendering.
#[derive(Clone, Debug)]
pub struct MediaService {
    #[debug(skip)]
    pub(crate) connection: Connection,
    #[debug(skip)]
    pub(crate) players: Arc<RwLock<HashMap<PlayerId, Arc<Player>>>>,
    #[debug(skip)]
    pub(crate) cancellation_token: CancellationToken,
    /// All discovered media players.
    pub player_list: Property<Vec<Arc<Player>>>,
    /// Currently active media player.
    pub active_player: Property<Option<Arc<Player>>>,
    /// Patterns for media players to ignore.
    pub ignored_patterns: Vec<String>,
}

impl MediaService {
    /// Creates a new MediaService with default configuration.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::InitializationFailed` if D-Bus connection fails
    #[instrument]
    pub async fn new() -> Result<Self, Error> {
        Self::builder().build().await
    }

    /// Creates a builder for configuring a MediaService.
    pub fn builder() -> MediaServiceBuilder {
        MediaServiceBuilder::new()
    }

    /// Get a snapshot of a specific media player's current state.
    ///
    /// Returns a non-monitored player instance representing the current state
    /// at the time of the call. The returned player's properties will not
    /// update when the actual player state changes.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::PlayerNotFound` if the player doesn't exist.
    /// Returns `MediaError::DbusError` if D-Bus operations fail.
    pub async fn player(&self, player_id: &PlayerId) -> Result<Player, Error> {
        Player::get(PlayerParams {
            connection: &self.connection,
            player_id: player_id.clone(),
        })
        .await
    }

    /// Get a live-updating instance of a specific media player.
    ///
    /// Returns a monitored player instance that automatically updates its
    /// properties when the actual player state changes. Use this when you
    /// need to track ongoing changes to a player's state.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::PlayerNotFound` if the player doesn't exist.
    /// Returns `MediaError::DbusError` if D-Bus operations fail.
    pub async fn player_monitored(&self, player_id: &PlayerId) -> Result<Arc<Player>, Error> {
        Player::get_live(LivePlayerParams {
            connection: &self.connection,
            player_id: player_id.clone(),
            cancellation_token: &self.cancellation_token,
        })
        .await
    }

    /// Get the current list of available media players.
    ///
    /// Returns a snapshot of all currently available MPRIS players,
    /// excluding any that match the ignored patterns configured at startup.
    pub fn players(&self) -> Vec<Arc<Player>> {
        self.player_list.get()
    }

    /// Get a stream that emits updates when the player list changes.
    ///
    /// Returns a stream that emits the updated player list whenever
    /// players are added or removed from the system.
    pub fn players_monitored(&self) -> impl Stream<Item = Vec<Arc<Player>>> + Send {
        self.player_list.watch()
    }

    /// Get the currently active media player.
    ///
    /// Returns the player that is currently set as active, or None if
    /// no player is active.
    pub fn active_player(&self) -> Option<Arc<Player>> {
        self.active_player.get()
    }

    /// Get a stream that emits updates when the active player changes.
    ///
    /// Returns a stream that emits whenever a different player becomes
    /// active or when the active player is cleared.
    pub fn active_player_monitored(&self) -> impl Stream<Item = Option<Arc<Player>>> + Send {
        self.active_player.watch()
    }

    /// Set which media player should be considered active.
    ///
    /// Sets the specified player as the active one, or clears the active
    /// player if None is provided.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::PlayerNotFound` if the specified player doesn't exist.
    pub async fn set_active_player(&self, player_id: Option<PlayerId>) -> Result<(), Error> {
        let Some(ref id) = player_id else {
            self.active_player.set(None);
            return Ok(());
        };

        let players = self.players.read().await;

        let Some(found_player) = players.get(id) else {
            return Err(Error::PlayerNotFound(id.clone()));
        };

        self.active_player.set(Some(found_player.clone()));

        Ok(())
    }
}

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

impl Drop for MediaService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
