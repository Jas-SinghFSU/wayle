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

#[cfg(test)]
#[allow(unsafe_code)]
mod tests {
    use std::{env, sync::Mutex};

    use super::*;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_temp_env<F>(xdg_value: Option<&str>, home_value: Option<&str>, test_fn: F)
    where
        F: FnOnce(),
    {
        let _guard = ENV_LOCK.lock().unwrap();

        let xdg_key = "XDG_CONFIG_HOME";
        let home_key = "HOME";

        let original_xdg = env::var(xdg_key).ok();
        let original_home = env::var(home_key).ok();

        unsafe {
            if let Some(value) = xdg_value {
                env::set_var(xdg_key, value);
            } else {
                env::remove_var(xdg_key);
            }

            if let Some(value) = home_value {
                env::set_var(home_key, value);
            } else {
                env::remove_var(home_key);
            }
        }

        test_fn();

        unsafe {
            if let Some(value) = original_xdg {
                env::set_var(xdg_key, value);
            } else {
                env::remove_var(xdg_key);
            }

            if let Some(value) = original_home {
                env::set_var(home_key, value);
            } else {
                env::remove_var(home_key);
            }
        }
    }

    #[test]
    fn state_file_path_uses_xdg_config_home_when_set() {
        with_temp_env(Some("/custom/config"), Some("/home/user"), || {
            let path = RuntimeState::state_file_path().unwrap();

            assert_eq!(
                path,
                PathBuf::from("/custom/config/wayle/runtime-state.json")
            );
        });
    }

    #[test]
    fn state_file_path_falls_back_to_home_config_when_xdg_not_set() {
        with_temp_env(None, Some("/home/testuser"), || {
            let path = RuntimeState::state_file_path().unwrap();

            assert_eq!(
                path,
                PathBuf::from("/home/testuser/.config/wayle/runtime-state.json")
            );
        });
    }

    #[test]
    fn state_file_path_returns_error_when_neither_env_var_set() {
        with_temp_env(None, None, || {
            let result = RuntimeState::state_file_path();

            assert!(result.is_err());
            let err = result.unwrap_err();
            assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
        });
    }

    #[test]
    fn load_returns_default_when_file_does_not_exist() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().to_str().unwrap();

        with_temp_env(Some(config_path), None, || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let result = rt.block_on(async { RuntimeState::load().await });

            assert!(result.is_ok());
            let state = result.unwrap();
            assert!(state.active_media_player.is_none());
        });
    }

    #[test]
    fn load_returns_parsed_state_when_file_exists_with_valid_json() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path();

        with_temp_env(Some(config_path.to_str().unwrap()), None, || {
            let state_dir = config_path.join("wayle");
            fs::create_dir_all(&state_dir).unwrap();

            let state_file = state_dir.join("runtime-state.json");
            let test_state = RuntimeState {
                active_media_player: Some("test-player".to_string()),
                last_updated: SystemTime::now(),
            };
            let json = serde_json::to_string_pretty(&test_state).unwrap();
            fs::write(state_file, json).unwrap();

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let result = rt.block_on(async { RuntimeState::load().await });

            assert!(result.is_ok());
            let loaded_state = result.unwrap();
            assert_eq!(
                loaded_state.active_media_player,
                Some("test-player".to_string())
            );
        });
    }

    #[test]
    fn load_returns_default_when_file_contains_invalid_json() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path();

        with_temp_env(Some(config_path.to_str().unwrap()), None, || {
            let state_dir = config_path.join("wayle");
            fs::create_dir_all(&state_dir).unwrap();

            let state_file = state_dir.join("runtime-state.json");
            fs::write(state_file, "{ invalid json }").unwrap();

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let result = rt.block_on(async { RuntimeState::load().await });

            assert!(result.is_ok());
            let state = result.unwrap();
            assert!(state.active_media_player.is_none());
        });
    }

    #[test]
    fn load_returns_error_when_env_vars_not_set() {
        with_temp_env(None, None, || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let result = rt.block_on(async { RuntimeState::load().await });

            assert!(result.is_err());
        });
    }

    #[test]
    fn save_creates_parent_directory_when_missing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path();

        with_temp_env(Some(config_path.to_str().unwrap()), None, || {
            let state = RuntimeState::default();

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let result = rt.block_on(async { state.save().await });

            assert!(result.is_ok());

            let state_dir = config_path.join("wayle");
            assert!(state_dir.exists());
            assert!(state_dir.join("runtime-state.json").exists());
        });
    }

    #[test]
    fn save_writes_serialized_state_to_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path();

        with_temp_env(Some(config_path.to_str().unwrap()), None, || {
            let state = RuntimeState {
                active_media_player: Some("my-player".to_string()),
                last_updated: SystemTime::now(),
            };

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let result = rt.block_on(async { state.save().await });

            assert!(result.is_ok());

            let state_file = config_path.join("wayle").join("runtime-state.json");
            let content = fs::read_to_string(state_file).unwrap();
            let loaded: RuntimeState = serde_json::from_str(&content).unwrap();

            assert_eq!(loaded.active_media_player, Some("my-player".to_string()));
        });
    }

    #[test]
    fn save_returns_error_when_env_vars_not_set() {
        with_temp_env(None, None, || {
            let state = RuntimeState::default();

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let result = rt.block_on(async { state.save().await });

            assert!(result.is_err());
        });
    }

    #[test]
    fn get_active_player_returns_none_when_no_state_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path();

        with_temp_env(Some(config_path.to_str().unwrap()), None, || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let result = rt.block_on(async { RuntimeState::get_active_player().await });

            assert!(result.is_ok());
            assert!(result.unwrap().is_none());
        });
    }

    #[test]
    fn get_active_player_returns_player_id_from_saved_state() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path();

        with_temp_env(Some(config_path.to_str().unwrap()), None, || {
            let state = RuntimeState {
                active_media_player: Some("saved-player".to_string()),
                last_updated: SystemTime::now(),
            };

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                state.save().await.unwrap();

                let result = RuntimeState::get_active_player().await;

                assert!(result.is_ok());
                assert_eq!(result.unwrap(), Some("saved-player".to_string()));
            });
        });
    }

    #[test]
    fn set_active_player_updates_player_id_and_timestamp() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path();

        with_temp_env(Some(config_path.to_str().unwrap()), None, || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                let before_timestamp = SystemTime::now();

                RuntimeState::set_active_player(Some("new-player".to_string()))
                    .await
                    .unwrap();

                let loaded = RuntimeState::load().await.unwrap();

                assert_eq!(loaded.active_media_player, Some("new-player".to_string()));
                assert!(loaded.last_updated >= before_timestamp);
            });
        });
    }

    #[test]
    fn set_active_player_persists_state_to_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path();

        with_temp_env(Some(config_path.to_str().unwrap()), None, || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                RuntimeState::set_active_player(Some("persistent-player".to_string()))
                    .await
                    .unwrap();

                let state_file = config_path.join("wayle").join("runtime-state.json");
                assert!(state_file.exists());

                let content = fs::read_to_string(state_file).unwrap();
                let loaded: RuntimeState = serde_json::from_str(&content).unwrap();

                assert_eq!(
                    loaded.active_media_player,
                    Some("persistent-player".to_string())
                );
            });
        });
    }
}
