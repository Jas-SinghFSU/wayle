//! D-Bus server interface implementation.

use std::sync::Arc;

use tracing::instrument;
use zbus::{fdo, interface};

use crate::{
    service::MediaService,
    types::{LoopMode, PlayerId, ShuffleMode},
};

/// D-Bus daemon for external control of the media service.
#[derive(Debug)]
pub(crate) struct MediaDaemon {
    pub service: Arc<MediaService>,
}

impl MediaDaemon {
    fn resolve_player(&self, player_id: &str) -> fdo::Result<PlayerId> {
        if player_id.is_empty() {
            self.service
                .active_player()
                .map(|p| p.id.clone())
                .ok_or_else(|| fdo::Error::Failed("No active player".to_string()))
        } else {
            Ok(PlayerId::from_bus_name(player_id))
        }
    }

    fn parse_loop_mode(mode: &str) -> fdo::Result<LoopMode> {
        match mode.to_lowercase().as_str() {
            "none" => Ok(LoopMode::None),
            "track" => Ok(LoopMode::Track),
            "playlist" => Ok(LoopMode::Playlist),
            _ => Err(fdo::Error::InvalidArgs(format!(
                "Invalid loop mode: {mode}. Expected: none, track, playlist"
            ))),
        }
    }

    fn parse_shuffle_state(state: &str) -> fdo::Result<Option<ShuffleMode>> {
        match state.to_lowercase().as_str() {
            "on" => Ok(Some(ShuffleMode::On)),
            "off" => Ok(Some(ShuffleMode::Off)),
            "toggle" => Ok(None),
            _ => Err(fdo::Error::InvalidArgs(format!(
                "Invalid shuffle state: {state}. Expected: on, off, toggle"
            ))),
        }
    }
}

#[interface(name = "com.wayle.Media1")]
impl MediaDaemon {
    /// Toggles play/pause for a player.
    ///
    /// Pass an empty string for `player_id` to use the active player.
    #[instrument(skip(self), fields(player = %player_id))]
    pub async fn play_pause(&self, player_id: String) -> fdo::Result<()> {
        let id = self.resolve_player(&player_id)?;

        let player = self
            .service
            .player(&id)
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))?;

        player
            .play_pause()
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))
    }

    /// Skips to the next track.
    ///
    /// Pass an empty string for `player_id` to use the active player.
    #[instrument(skip(self), fields(player = %player_id))]
    pub async fn next(&self, player_id: String) -> fdo::Result<()> {
        let id = self.resolve_player(&player_id)?;

        let player = self
            .service
            .player(&id)
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))?;

        player
            .next()
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))
    }

    /// Goes to the previous track.
    ///
    /// Pass an empty string for `player_id` to use the active player.
    #[instrument(skip(self), fields(player = %player_id))]
    pub async fn previous(&self, player_id: String) -> fdo::Result<()> {
        let id = self.resolve_player(&player_id)?;

        let player = self
            .service
            .player(&id)
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))?;

        player
            .previous()
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))
    }

    /// Seeks to a position in microseconds.
    ///
    /// Pass an empty string for `player_id` to use the active player.
    #[instrument(skip(self), fields(player = %player_id, position_us = %position_us))]
    pub async fn seek(&self, player_id: String, position_us: i64) -> fdo::Result<()> {
        let id = self.resolve_player(&player_id)?;

        let player = self
            .service
            .player(&id)
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))?;

        let position = std::time::Duration::from_micros(position_us.max(0) as u64);

        player
            .set_position(position)
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))
    }

    /// Sets the shuffle mode for a player.
    ///
    /// `state` must be one of: "on", "off", "toggle".
    /// Pass an empty string for `player_id` to use the active player.
    #[instrument(skip(self), fields(player = %player_id, state = %state))]
    pub async fn set_shuffle(&self, player_id: String, state: String) -> fdo::Result<()> {
        let id = self.resolve_player(&player_id)?;
        let mode = Self::parse_shuffle_state(&state)?;

        let player = self
            .service
            .player(&id)
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))?;

        match mode {
            Some(m) => player
                .set_shuffle_mode(m)
                .await
                .map_err(|e| fdo::Error::Failed(e.to_string())),
            None => player
                .toggle_shuffle()
                .await
                .map_err(|e| fdo::Error::Failed(e.to_string())),
        }
    }

    /// Sets the loop mode for a player.
    ///
    /// `mode` must be one of: "none", "track", "playlist".
    /// Pass an empty string for `player_id` to use the active player.
    #[instrument(skip(self), fields(player = %player_id, mode = %mode))]
    pub async fn set_loop_status(&self, player_id: String, mode: String) -> fdo::Result<()> {
        let id = self.resolve_player(&player_id)?;
        let loop_mode = Self::parse_loop_mode(&mode)?;

        let player = self
            .service
            .player(&id)
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))?;

        player
            .set_loop_mode(loop_mode)
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))
    }

    /// Lists all available media players.
    ///
    /// Returns a list of tuples: (player_id, identity, playback_state).
    #[instrument(skip(self))]
    pub async fn list_players(&self) -> Vec<(String, String, String)> {
        self.service
            .players()
            .into_iter()
            .map(|player| {
                let id = player.id.to_string();
                let identity = player.identity.get();
                let state = format!("{:?}", player.playback_state.get());
                (id, identity, state)
            })
            .collect()
    }

    /// Gets the active player ID.
    ///
    /// Returns an empty string if no player is active.
    #[instrument(skip(self))]
    pub async fn get_active_player(&self) -> String {
        self.service
            .active_player()
            .map(|p| p.id.to_string())
            .unwrap_or_default()
    }

    /// Sets the active player by ID.
    ///
    /// Pass an empty string to clear the active player.
    #[instrument(skip(self), fields(player = %player_id))]
    pub async fn set_active_player(&self, player_id: String) -> fdo::Result<()> {
        let id = if player_id.is_empty() {
            None
        } else {
            Some(PlayerId::from_bus_name(&player_id))
        };

        self.service
            .set_active_player(id)
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))
    }

    /// Gets detailed information about a player.
    ///
    /// Returns a dictionary with player metadata.
    /// Pass an empty string for `player_id` to use the active player.
    #[instrument(skip(self), fields(player = %player_id))]
    pub async fn get_player_info(
        &self,
        player_id: String,
    ) -> fdo::Result<std::collections::HashMap<String, String>> {
        let id = self.resolve_player(&player_id)?;

        let player = self
            .service
            .player(&id)
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))?;

        let mut info = std::collections::HashMap::new();
        info.insert("id".to_string(), player.id.to_string());
        info.insert("identity".to_string(), player.identity.get());
        info.insert(
            "playback_state".to_string(),
            format!("{:?}", player.playback_state.get()),
        );
        info.insert(
            "loop_mode".to_string(),
            format!("{:?}", player.loop_mode.get()),
        );
        info.insert(
            "shuffle_mode".to_string(),
            format!("{:?}", player.shuffle_mode.get()),
        );
        info.insert(
            "volume".to_string(),
            format!("{:.0}", player.volume.get().as_percentage()),
        );
        info.insert(
            "can_go_next".to_string(),
            player.can_go_next.get().to_string(),
        );
        info.insert(
            "can_go_previous".to_string(),
            player.can_go_previous.get().to_string(),
        );
        info.insert("can_seek".to_string(), player.can_seek.get().to_string());
        info.insert("can_loop".to_string(), player.can_loop.get().to_string());
        info.insert(
            "can_shuffle".to_string(),
            player.can_shuffle.get().to_string(),
        );

        info.insert("title".to_string(), player.metadata.title.get());
        info.insert("artist".to_string(), player.metadata.artist.get());
        info.insert("album".to_string(), player.metadata.album.get());

        if let Some(art_url) = player.metadata.art_url.get() {
            info.insert("art_url".to_string(), art_url);
        }
        if let Some(length) = player.metadata.length.get() {
            info.insert("length_us".to_string(), length.as_micros().to_string());
        }

        Ok(info)
    }

    /// The currently active player ID.
    #[zbus(property)]
    pub async fn active_player(&self) -> String {
        self.service
            .active_player()
            .map(|p| p.id.to_string())
            .unwrap_or_default()
    }

    /// Number of available players.
    #[zbus(property)]
    pub async fn player_count(&self) -> u32 {
        self.service.players().len() as u32
    }
}
