use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use toml::Value;
use tracing::{debug, info, instrument, warn};

/// Thread-safe storage for configuration
pub type ConfigData = Arc<RwLock<Config>>;

/// Thread-safe storage for runtime configuration values
pub type RuntimeConfig = Arc<RwLock<HashMap<String, Value>>>;

use super::{
    ConfigChange, ConfigError, Subscription,
    broadcast::BroadcastService,
    path_ops::{navigate_path, set_value_at_path},
};
use crate::config::{Config, ConfigPaths};

/// Thread-safe configuration store with reactive change notifications.
///
/// Provides centralized configuration management with the ability to
/// read, write, and observe configuration changes across the application.
#[derive(Clone)]
pub struct ConfigRuntime {
    config: ConfigData,
    broadcast_service: BroadcastService,
    runtime_config: RuntimeConfig,
}

impl Default for ConfigRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigRuntime {
    /// Creates a new ConfigRuntime with default configuration values.
    pub fn new() -> Self {
        let config = Config::default();
        let broadcast_service = BroadcastService::new();

        Self {
            config: Arc::new(RwLock::new(config)),
            runtime_config: Arc::new(RwLock::new(HashMap::new())),
            broadcast_service,
        }
    }

    /// Loads a ConfigRuntime from the main configuration file.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::ProcessingError` if the configuration file cannot be loaded.
    #[instrument]
    pub fn load() -> Result<Self, ConfigError> {
        let main_config = ConfigPaths::main_config();
        info!("Loading configuration from {}", main_config.display());

        let config =
            Config::load_with_imports(&main_config).map_err(|e| ConfigError::ProcessingError {
                operation: "load config".to_string(),
                details: e.to_string(),
            })?;
        let broadcast_service = BroadcastService::new();

        debug!("Loading runtime configuration");
        let runtime_config = Self::load_runtime_config()?;

        info!("Configuration loaded successfully");
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            runtime_config: Arc::new(RwLock::new(runtime_config)),
            broadcast_service,
        })
    }

    /// Sets a configuration value at the specified path and broadcasts the change
    ///
    /// # Arguments
    /// * `path` - Dot-separated path to the configuration field (e.g., "server.port")
    /// * `value` - The new TOML value to set at the path
    ///
    /// # Errors
    /// * `ConfigError::InvalidPath` - If the path doesn't exist
    /// * `ConfigError::LockError` - If the write lock cannot be acquired
    /// * `ConfigError::SerializationError` - If the config cannot be serialized
    /// * `ConfigError::ConversionError` - If the config cannot be converted between formats
    /// * `ConfigError::PersistenceError` - If the config cannot be saved to disk
    #[instrument(skip(self, value), fields(path = %path))]
    pub fn set_by_path(&self, path: &str, value: Value) -> Result<(), ConfigError> {
        let old_value = self.get_by_path(path).ok();
        debug!("Setting config value at path: {}", path);

        self.runtime_config
            .write()
            .map_err(|e| ConfigError::LockError {
                lock_type: "write".to_string(),
                details: format!("Failed to acquire write lock for runtime_config: {e}"),
            })?
            .insert(path.to_string(), value.clone());

        {
            let mut config = self.config.write().map_err(|_| ConfigError::LockError {
                lock_type: "write".to_string(),
                details: "Failed to acquire write lock".to_string(),
            })?;

            self.set_config_field(&mut config, path, &value)?;
        }

        debug!("Persisting configuration changes");
        self.save_config()?;

        let change = ConfigChange::new(path.to_string(), old_value, value);

        self.broadcast_change(change);
        Ok(())
    }

    /// Retrieves a configuration value at the specified path
    ///
    /// # Arguments
    /// * `path` - Dot-separated path to the configuration field (e.g., "server.port")
    ///
    /// # Errors
    /// * `ConfigError::InvalidPath` - If the path doesn't exist
    /// * `ConfigError::LockError` - If the read lock cannot be acquired
    /// * `ConfigError::SerializationError` - If the config cannot be serialized
    /// * `ConfigError::ConversionError` - If the config cannot be converted to TOML Value
    pub fn get_by_path(&self, path: &str) -> Result<Value, ConfigError> {
        let config = self.config.read().map_err(|_| ConfigError::LockError {
            lock_type: "read".to_string(),
            details: "Failed to acquire read lock".to_string(),
        })?;

        Self::get_config_field(&config, path)
    }

    /// Returns a clone of the current configuration, handling poisoned locks gracefully
    pub fn get_current(&self) -> Config {
        match self.config.read() {
            Ok(guard) => guard.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        }
    }

    /// Subscribe to configuration changes matching the specified path pattern.
    ///
    /// Returns a receiver that will receive only changes matching the pattern.
    /// Events are filtered at the source for efficiency with many subscribers.
    ///
    /// # Arguments
    /// * `pattern` - A pattern to match configuration paths (supports "*" wildcards)
    ///
    /// # Errors
    /// Returns `ConfigError::ServiceUnavailable` if the broadcast service is unavailable.
    pub async fn subscribe_to_path(&self, pattern: &str) -> Result<Subscription, ConfigError> {
        self.broadcast_service.subscribe(pattern).await
    }

    /// Saves the current configuration to the runtime config file
    ///
    /// # Errors
    /// * `ConfigError::LockError` - If the read lock cannot be acquired
    /// * `ConfigError::SerializationError` - If the config cannot be serialized to TOML
    /// * `ConfigError::PersistenceError` - If the configuration cannot be saved to disk
    /// * `ConfigError::IoError` - If the main config file cannot be read
    pub fn save_config(&self) -> Result<(), ConfigError> {
        let config_data = {
            self.runtime_config
                .read()
                .map_err(|_| ConfigError::LockError {
                    lock_type: "read".to_string(),
                    details: "Failed to acquire read lock".to_string(),
                })?
                .clone()
        };

        let mut runtime_value = Value::Table(toml::Table::new());

        for (path, value) in config_data.iter() {
            set_value_at_path(&mut runtime_value, path, value.clone())?;
        }

        let config_path = ConfigPaths::runtime_config();
        let temp_path = config_path.with_extension("tmp");

        let toml_str = toml::to_string_pretty(&runtime_value).map_err(|e| {
            ConfigError::SerializationError {
                content_type: "config".to_string(),
                details: e.to_string(),
            }
        })?;

        Self::ensure_config_dir()?;

        fs::write(&temp_path, toml_str).map_err(|e| ConfigError::PersistenceError {
            path: temp_path.clone(),
            details: e.to_string(),
        })?;

        fs::rename(temp_path, &config_path).map_err(|e| ConfigError::PersistenceError {
            path: config_path.clone(),
            details: e.to_string(),
        })?;

        let main_path = ConfigPaths::main_config();
        let mut main_config_toml =
            fs::read_to_string(&main_path).map_err(|_| ConfigError::IoError {
                path: main_path.clone(),
                details: "Main config file not found during persist operation".to_string(),
            })?;

        if !main_config_toml.contains("\"@runtime\"") {
            main_config_toml = Self::ensure_runtime_import(&main_config_toml);
            fs::write(&main_path, main_config_toml).map_err(|e| ConfigError::PersistenceError {
                path: main_path.clone(),
                details: format!("Failed to add runtime import to main config: {e}"),
            })?;
        }

        Ok(())
    }

    pub(super) fn broadcast_change(&self, change: ConfigChange) {
        let broadcast_service = self.broadcast_service.clone();
        tokio::spawn(async move {
            if let Err(e) = broadcast_service.broadcast(change).await {
                eprintln!("Warning: Failed to broadcast config change: {e}");
            }
        });
    }

    pub(super) fn update_config(&self, new_config: Config) -> Result<(), ConfigError> {
        let mut config_guard = self.config.write().map_err(|e| ConfigError::LockError {
            lock_type: "write".to_string(),
            details: format!("Failed to acquire write lock: {e}"),
        })?;
        *config_guard = new_config;
        Ok(())
    }

    fn set_config_field(
        &self,
        config: &mut Config,
        path: &str,
        value: &Value,
    ) -> Result<(), ConfigError> {
        let mut config_value =
            Value::try_from(config.clone()).map_err(|e| ConfigError::SerializationError {
                content_type: "config".to_string(),
                details: e.to_string(),
            })?;

        set_value_at_path(&mut config_value, path, value.clone())?;

        *config = config_value
            .try_into()
            .map_err(|e| ConfigError::ConversionError {
                from: "toml::Value".to_string(),
                to: "Config".to_string(),
                details: e.to_string(),
            })?;

        Ok(())
    }

    fn get_config_field(config: &Config, path: &str) -> Result<Value, ConfigError> {
        let config_value =
            Value::try_from(config.clone()).map_err(|e| ConfigError::SerializationError {
                content_type: "config".to_string(),
                details: e.to_string(),
            })?;

        navigate_path(&config_value, path)
    }

    fn ensure_runtime_import(config: &str) -> String {
        let mut doc: Value = toml::from_str(config).unwrap_or_else(|_| {
            let mut table = toml::map::Map::new();
            table.insert("imports".to_string(), Value::Array(vec![]));

            Value::Table(table)
        });

        if let Value::Table(table) = &mut doc {
            let imports = table
                .entry("imports")
                .or_insert_with(|| Value::Array(vec![]));

            if let Value::Array(arr) = imports {
                let runtime_import = Value::String("@runtime".to_string());

                if !arr.iter().any(|v| v.as_str() == Some("@runtime")) {
                    arr.push(runtime_import);
                }
            }
        }

        toml::to_string(&doc).unwrap_or_else(|_| config.to_string())
    }

    fn load_runtime_config() -> Result<HashMap<String, Value>, ConfigError> {
        let runtime_path = ConfigPaths::runtime_config();
        if runtime_path.exists() {
            let runtime_config =
                fs::read_to_string(&runtime_path).map_err(|e| ConfigError::IoError {
                    path: runtime_path.clone(),
                    details: format!("Failed to read runtime.toml: {e}"),
                })?;

            let runtime_toml: Value =
                toml::from_str(&runtime_config).map_err(|e| ConfigError::TomlParseError {
                    location: runtime_path.to_string_lossy().to_string(),
                    details: e.to_string(),
                })?;

            let mut flat_map: HashMap<String, Value> = HashMap::new();

            Self::flatten_toml_to_paths(&runtime_toml, "", &mut flat_map);
            Ok(flat_map)
        } else {
            Ok(HashMap::new())
        }
    }

    fn flatten_toml_to_paths(value: &Value, prefix: &str, map: &mut HashMap<String, Value>) {
        match value {
            Value::Table(table) => {
                for (key, value) in table {
                    let path = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{prefix}.{key}")
                    };
                    Self::flatten_toml_to_paths(value, &path, map);
                }
            }
            _ => {
                map.insert(prefix.to_string(), value.clone());
            }
        }
    }

    fn ensure_config_dir() -> Result<(), ConfigError> {
        let config_dir = ConfigPaths::config_dir().map_err(|e| ConfigError::PersistenceError {
            path: PathBuf::from("."),
            details: format!("Failed to determine config directory: {e}"),
        })?;

        fs::create_dir_all(&config_dir).map_err(|e| ConfigError::PersistenceError {
            path: config_dir,
            details: format!("Failed to create config directory: {e}"),
        })?;

        Ok(())
    }
}
