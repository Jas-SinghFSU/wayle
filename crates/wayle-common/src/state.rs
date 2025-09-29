use std::{fs, io::Error, path::PathBuf, time::SystemTime};

use serde::{Deserialize, Serialize};

/// Runtime state that persists between service operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeState {
    /// Currently active media player ID
    pub active_media_player: Option<String>,

    /// Last time this state was updated
    pub last_updated: SystemTime,
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self {
            active_media_player: None,
            last_updated: SystemTime::now(),
        }
    }
}

impl RuntimeState {
    fn state_file_path() -> Result<PathBuf, Error> {
        let config_home = std::env::var("XDG_CONFIG_HOME")
            .or_else(|_| std::env::var("HOME").map(|home| format!("{home}/.config")))
            .map_err(|e| {
                Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Neither XDG_CONFIG_HOME nor HOME environment variable found: {e}"),
                )
            })?;

        Ok(PathBuf::from(config_home)
            .join("wayle")
            .join("runtime-state.json"))
    }

    /// Load runtime state from file, or create default if not exists
    ///
    /// # Errors
    /// Returns error if file cannot be read or config directory is inaccessible
    pub async fn load() -> Result<Self, Error> {
        let path = Self::state_file_path()?;

        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let state: Self = serde_json::from_str(&content).unwrap_or_else(|_| Self::default());
            Ok(state)
        } else {
            Ok(Self::default())
        }
    }

    /// Save runtime state to file
    ///
    /// # Errors
    /// Returns error if file cannot be written or directory cannot be created
    pub async fn save(&self) -> Result<(), Error> {
        let path = Self::state_file_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self).map_err(Error::other)?;
        fs::write(&path, content)?;

        Ok(())
    }

    /// Get the currently active media player
    ///
    /// # Errors
    /// Returns error if state file cannot be loaded
    pub async fn get_active_player() -> Result<Option<String>, Error> {
        let state = Self::load().await?;
        Ok(state.active_media_player)
    }

    /// Set the active media player and persist to file
    ///
    /// # Errors
    /// Returns error if state cannot be loaded or saved
    pub async fn set_active_player(player_id: Option<String>) -> Result<(), Error> {
        let mut state = Self::load().await?;
        state.active_media_player = player_id;
        state.last_updated = SystemTime::now();
        state.save().await?;

        Ok(())
    }
}
