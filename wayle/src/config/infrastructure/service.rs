use std::{
    collections::HashMap,
    fs,
    sync::{Arc, RwLock},
};

use tracing::{info, instrument};
use wayle_common::UpdateFromToml;

use super::{error::Error, paths::ConfigPaths, toml_path, watcher::FileWatcher};
use crate::config::Config;

/// Configuration service with reactive properties.
///
/// Provides strongly-typed access to all configuration values
/// with the same Property<T> pattern used by other services.
/// Each config field can be watched independently for changes.
///
/// Only values that differ from defaults are persisted to runtime.toml.
#[derive(Clone)]
pub struct ConfigService {
    config: Arc<Config>,
    runtime_config: Arc<RwLock<HashMap<String, toml::Value>>>,
    _watcher: Arc<RwLock<Option<FileWatcher>>>,
}

impl ConfigService {
    /// Initialize config service from TOML files.
    ///
    /// Loads config from disk using the existing import/merge logic.
    /// Automatically creates default config files if they don't exist.
    /// Starts file watcher to sync disk changes to Properties.
    ///
    /// # Errors
    ///
    /// Returns error if config files cannot be loaded or parsed.
    #[instrument]
    pub async fn load() -> Result<Arc<Self>, Error> {
        info!("Loading configuration");

        let config_path = ConfigPaths::main_config();
        let config = Config::load_with_imports(&config_path)?;
        let runtime_config = Self::load_runtime_config()?;

        let service = Arc::new(Self {
            config: Arc::new(config),
            runtime_config: Arc::new(RwLock::new(runtime_config)),
            _watcher: Arc::new(RwLock::new(None)),
        });

        let watcher = FileWatcher::start(Arc::clone(&service))?;
        *service
            ._watcher
            .write()
            .map_err(|e| Error::PersistenceError {
                path: config_path,
                details: format!("Failed to initialize watcher: {e}"),
            })? = Some(watcher);

        info!("Configuration loaded successfully");

        Ok(service)
    }

    /// Get reference to the config.
    ///
    /// This provides access to all reactive configuration fields.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Serialize current config to TOML string.
    ///
    /// # Errors
    ///
    /// Returns error if serialization fails.
    pub fn to_toml(&self) -> Result<String, Error> {
        toml::to_string_pretty(self.config.as_ref()).map_err(|e| Error::SerializationError {
            content_type: String::from("config"),
            details: e.to_string(),
        })
    }

    fn to_toml_value(&self) -> Result<toml::Value, Error> {
        toml::Value::try_from(self.config.as_ref()).map_err(|e| Error::SerializationError {
            content_type: String::from("config"),
            details: e.to_string(),
        })
    }

    /// Save changed config values to runtime.toml.
    ///
    /// Only saves values that have been explicitly set, not defaults.
    /// Ensures config.toml imports runtime.toml.
    ///
    /// # Errors
    ///
    /// Returns error if config cannot be serialized or written to disk.
    #[instrument(skip(self))]
    pub async fn save(&self) -> Result<(), Error> {
        let runtime_map = self
            .runtime_config
            .read()
            .map_err(|e| Error::PersistenceError {
                path: ConfigPaths::runtime_config(),
                details: format!("Failed to acquire read lock: {e}"),
            })?;

        let mut runtime_value = toml::Value::Table(toml::Table::new());
        for (path, value) in runtime_map.iter() {
            toml_path::insert(&mut runtime_value, path, value.clone())?;
        }

        let runtime_path = ConfigPaths::runtime_config();
        let temp_path = runtime_path.with_extension("tmp");

        let toml_str =
            toml::to_string_pretty(&runtime_value).map_err(|e| Error::SerializationError {
                content_type: String::from("runtime config"),
                details: e.to_string(),
            })?;

        fs::write(&temp_path, toml_str).map_err(|e| Error::PersistenceError {
            path: temp_path.clone(),
            details: e.to_string(),
        })?;

        fs::rename(&temp_path, &runtime_path).map_err(|e| Error::PersistenceError {
            path: runtime_path.clone(),
            details: e.to_string(),
        })?;

        Self::ensure_runtime_import()?;

        info!("Configuration saved to runtime.toml");

        Ok(())
    }

    /// Get a configuration value by path.
    ///
    /// # Arguments
    ///
    /// * `path` - Dot-separated path (e.g., "battery.enabled", "clock.general.format")
    ///
    /// # Errors
    ///
    /// Returns error if path is invalid or value cannot be retrieved.
    pub fn get_by_path(&self, path: &str) -> Result<toml::Value, Error> {
        let mut value = self.to_toml_value()?;

        for segment in path.split('.') {
            value = value
                .get(segment)
                .ok_or_else(|| Error::InvalidConfigField {
                    field: segment.to_string(),
                    component: path.to_string(),
                    reason: String::from("field not found in config"),
                })?
                .clone();
        }

        Ok(value)
    }

    /// Set a configuration value by path.
    ///
    /// Updates the Property at the specified path and adds the value to runtime config tracking.
    ///
    /// # Arguments
    ///
    /// * `path` - Dot-separated path (e.g., "battery.enabled")
    /// * `value` - TOML value to set
    ///
    /// # Errors
    ///
    /// Returns error if path is invalid or value cannot be set.
    pub fn set_by_path(&self, path: &str, value: toml::Value) -> Result<(), Error> {
        self.runtime_config
            .write()
            .map_err(|e| Error::PersistenceError {
                path: ConfigPaths::runtime_config(),
                details: format!("Failed to acquire write lock: {e}"),
            })?
            .insert(path.to_string(), value.clone());

        let mut root = self.to_toml_value()?;

        toml_path::insert(&mut root, path, value)?;
        self.config.update_from_toml(&root);

        Ok(())
    }

    fn load_runtime_config() -> Result<HashMap<String, toml::Value>, Error> {
        let runtime_path = ConfigPaths::runtime_config();
        if !runtime_path.exists() {
            return Ok(HashMap::new());
        }

        let runtime_content = fs::read_to_string(&runtime_path).map_err(|e| Error::IoError {
            path: runtime_path.clone(),
            details: e.to_string(),
        })?;

        let runtime_toml: toml::Value = toml::from_str(&runtime_content)
            .map_err(|e| Error::toml_parse(e, Some(&runtime_path)))?;

        let mut flat_map = HashMap::new();
        toml_path::flatten(&runtime_toml, "", &mut flat_map);

        Ok(flat_map)
    }

    fn ensure_runtime_import() -> Result<(), Error> {
        let main_path = ConfigPaths::main_config();
        let main_content = fs::read_to_string(&main_path).map_err(|e| Error::IoError {
            path: main_path.clone(),
            details: e.to_string(),
        })?;

        if main_content.contains("@runtime") {
            return Ok(());
        }

        let updated_content = format!("imports = [\"@runtime\"]\n\n{main_content}");
        fs::write(&main_path, updated_content).map_err(|e| Error::PersistenceError {
            path: main_path,
            details: e.to_string(),
        })?;

        Ok(())
    }
}
