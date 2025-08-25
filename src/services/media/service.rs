use std::{collections::HashMap, sync::Arc};

use futures::Stream;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::{info, instrument};
use zbus::Connection;

use super::{MediaError, PlayerId, core::Player, monitoring::MprisMonitoring};
use crate::services::common::Property;

/// Configuration for the MPRIS service
#[derive(Default)]
pub struct Config {
    /// Patterns to ignore when discovering players
    pub ignored_players: Vec<String>,
}

/// MPRIS service with reactive property-based architecture.
///
/// Provides fine-grained reactive updates for efficient UI rendering.
#[derive(Clone)]
pub struct MediaService {
    connection: Connection,
    players: Arc<RwLock<HashMap<PlayerId, Arc<Player>>>>,
    player_list: Property<Vec<Arc<Player>>>,
    active_player: Property<Option<Arc<Player>>>,
    ignored_patterns: Vec<String>,
    cancellation_token: CancellationToken,
}

impl MediaService {
    /// Create a new MPRIS service (compatibility method).
    ///
    /// This is a compatibility method for the old API. Prefer using `start()` instead.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::InitializationFailed` if D-Bus connection fails
    pub async fn new(ignored_players: Vec<String>) -> Result<Self, MediaError> {
        Self::start(Config { ignored_players }).await
    }

    /// Start the MPRIS service with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::InitializationFailed` if D-Bus connection fails
    #[instrument(skip(config))]
    pub async fn start(config: Config) -> Result<Self, MediaError> {
        info!("Starting MPRIS service with property-based architecture");

        let connection = Connection::session().await.map_err(|e| {
            MediaError::InitializationFailed(format!("D-Bus connection failed: {e}"))
        })?;

        let cancellation_token = CancellationToken::new();

        let service = Self {
            connection,
            players: Arc::new(RwLock::new(HashMap::new())),
            player_list: Property::new(Vec::new()),
            active_player: Property::new(None),
            ignored_patterns: config.ignored_players,
            cancellation_token: cancellation_token.clone(),
        };

        MprisMonitoring::start(
            &service.connection,
            Arc::clone(&service.players),
            service.player_list.clone(),
            service.active_player.clone(),
            service.ignored_patterns.clone(),
            cancellation_token.child_token(),
        )
        .await?;

        Ok(service)
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
    pub async fn player(&self, player_id: &PlayerId) -> Result<Arc<Player>, MediaError> {
        Player::get(&self.connection, player_id.clone()).await
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
    pub async fn player_monitored(&self, player_id: &PlayerId) -> Result<Arc<Player>, MediaError> {
        Player::get_live(
            &self.connection,
            player_id.clone(),
            self.cancellation_token.child_token(),
        )
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
    pub async fn set_active_player(&self, player_id: Option<PlayerId>) -> Result<(), MediaError> {
        if let Some(ref id) = player_id {
            let players = self.players.read().await;
            if !players.contains_key(id) {
                return Err(MediaError::PlayerNotFound(id.clone()));
            }
        }

        let player = match player_id {
            Some(ref id) => self.player(id).await.ok(),
            None => None,
        };

        self.active_player.set(player);

        Ok(())
    }
}

impl Drop for MediaService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
