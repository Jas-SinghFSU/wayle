//! D-Bus client proxy for the media service.
#![allow(missing_docs)]

use std::collections::HashMap;

use zbus::{Result, proxy};

/// D-Bus client proxy for controlling the media service.
///
/// Connects to a running media daemon and allows external control
/// of media player playback, modes, and active player selection.
#[proxy(
    interface = "com.wayle.Media1",
    default_service = "com.wayle.Media1",
    default_path = "/com/wayle/Media",
    gen_blocking = false
)]
pub trait Media {
    /// Toggles play/pause for a player.
    ///
    /// An empty string for `player_id` targets the active player.
    async fn play_pause(&self, player_id: String) -> Result<()>;

    /// Skips to the next track.
    ///
    /// An empty string for `player_id` targets the active player.
    async fn next(&self, player_id: String) -> Result<()>;

    /// Goes to the previous track.
    ///
    /// An empty string for `player_id` targets the active player.
    async fn previous(&self, player_id: String) -> Result<()>;

    /// Seeks to a position in microseconds.
    ///
    /// An empty string for `player_id` targets the active player.
    async fn seek(&self, player_id: String, position_us: i64) -> Result<()>;

    /// Sets the shuffle mode for a player.
    ///
    /// `state` accepts: "on", "off", or "toggle".
    /// An empty string for `player_id` targets the active player.
    async fn set_shuffle(&self, player_id: String, state: String) -> Result<()>;

    /// Sets the loop mode for a player.
    ///
    /// `mode` accepts: "none", "track", or "playlist".
    /// An empty string for `player_id` targets the active player.
    async fn set_loop_status(&self, player_id: String, mode: String) -> Result<()>;

    /// Lists all available media players.
    ///
    /// Returns a list of tuples: (player_id, identity, playback_state).
    async fn list_players(&self) -> Result<Vec<(String, String, String)>>;

    /// Gets the active player ID.
    ///
    /// Returns an empty string if no player is active.
    async fn get_active_player(&self) -> Result<String>;

    /// Sets the active player by ID.
    ///
    /// An empty string clears the active player.
    async fn set_active_player(&self, player_id: String) -> Result<()>;

    /// Gets detailed information about a player.
    ///
    /// An empty string for `player_id` targets the active player.
    async fn get_player_info(&self, player_id: String) -> Result<HashMap<String, String>>;

    /// The currently active player ID.
    #[zbus(property)]
    fn active_player(&self) -> Result<String>;

    /// Number of available players.
    #[zbus(property)]
    fn player_count(&self) -> Result<u32>;
}
