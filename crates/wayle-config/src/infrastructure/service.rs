use std::{
    fs,
    sync::{Arc, RwLock},
};

use tracing::{info, instrument};
use wayle_common::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues};

use super::{error::Error, paths::ConfigPaths, toml_path, watcher::FileWatcher};
use crate::{Config, infrastructure::themes::utils::load_themes};

/// Configuration service with reactive properties.
///
/// Provides strongly-typed access to all configuration values.
/// Each config field can be watched independently for changes.
/// Runtime overrides are extracted directly from ConfigProperty fields.
#[derive(Clone)]
pub struct ConfigService {
    config: Arc<Config>,
    _watcher: Arc<RwLock<Option<FileWatcher>>>,
}

impl ConfigService {
    /// Initialize config service from TOML files.
    ///
    /// Creates default config, applies config.toml to the config layer,
    /// and runtime.toml to the runtime layer. Starts file watcher for hot-reload.
    ///
    /// # Errors
    ///
    /// Returns error if config files cannot be loaded or parsed.
    #[instrument]
    pub async fn load() -> Result<Arc<Self>, Error> {
        info!("Loading configuration");

        let config = Config::default();
        let config_path = ConfigPaths::main_config();

        if let Ok(config_toml) = Self::load_toml_file(&config_path) {
            config.apply_config_layer(&config_toml);
        }

        let runtime_path = ConfigPaths::runtime_config();
        if let Ok(runtime_toml) = Self::load_toml_file(&runtime_path) {
            config.apply_runtime_layer(&runtime_toml);
        }

        let service = Arc::new(Self {
            config: Arc::new(config),
            _watcher: Arc::new(RwLock::new(None)),
        });

        let themes_dir = ConfigPaths::themes_dir();
        load_themes(&service.config, &themes_dir);

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
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Save runtime layer values to runtime.toml.
    ///
    /// Extracts only values with runtime overrides from the config tree.
    ///
    /// # Errors
    ///
    /// Returns error if config cannot be serialized or written to disk.
    #[instrument(skip(self))]
    pub async fn save(&self) -> Result<(), Error> {
        let runtime_value = self
            .config
            .extract_runtime_values()
            .unwrap_or_else(|| toml::Value::Table(toml::Table::new()));

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

        info!("Configuration saved to runtime.toml");

        Ok(())
    }

    fn load_toml_file(path: &std::path::Path) -> Result<toml::Value, Error> {
        let content = fs::read_to_string(path).map_err(|e| Error::IoError {
            path: path.to_path_buf(),
            details: e.to_string(),
        })?;

        toml::from_str(&content).map_err(|e| Error::toml_parse(e, Some(path)))
    }
}

/// CLI-specific extension methods for ConfigService.
///
/// These use string-based paths for dynamic config access from the command line.
/// Application code should use strongly-typed access via `config()`.
pub trait ConfigServiceCli {
    /// Get a configuration value by dot-separated path.
    ///
    /// # Errors
    ///
    /// Returns error if path is invalid.
    fn get_by_path(&self, path: &str) -> Result<toml::Value, Error>;

    /// Set a runtime override by dot-separated path.
    ///
    /// # Errors
    ///
    /// Returns error if path is invalid.
    fn set_by_path(&self, path: &str, value: toml::Value) -> Result<(), Error>;
}

impl ConfigServiceCli for ConfigService {
    fn get_by_path(&self, path: &str) -> Result<toml::Value, Error> {
        let config_value =
            toml::Value::try_from(self.config.as_ref()).map_err(|e| Error::SerializationError {
                content_type: String::from("config"),
                details: e.to_string(),
            })?;

        let mut value = config_value;
        for segment in path.split('.') {
            value = value
                .get(segment)
                .ok_or_else(|| Error::InvalidConfigField {
                    field: segment.to_string(),
                    component: path.to_string(),
                    reason: String::from("field not found"),
                })?
                .clone();
        }

        Ok(value)
    }

    fn set_by_path(&self, path: &str, value: toml::Value) -> Result<(), Error> {
        let mut root = toml::Value::Table(toml::Table::new());
        toml_path::insert(&mut root, path, value)?;
        self.config.apply_runtime_layer(&root);
        Ok(())
    }
}
