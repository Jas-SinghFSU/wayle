use std::{
    fs,
    sync::{Arc, RwLock},
};

use tracing::{info, instrument, warn};
use wayle_common::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues};

use super::{
    error::{Error, InvalidFieldReason, IoOperation},
    paths::ConfigPaths,
    toml_path,
    watcher::FileWatcher,
};
use crate::{Config, infrastructure::themes::utils::load_themes};

/// Reactive configuration service.
///
/// Each config field can be watched independently for changes. Runtime
/// overrides are extracted directly from `ConfigProperty` fields.
#[derive(Clone)]
pub struct ConfigService {
    config: Arc<Config>,
    _watcher: Arc<RwLock<Option<FileWatcher>>>,
}

impl ConfigService {
    /// Loads configuration from TOML files and starts file watcher.
    ///
    /// Applies `config.toml` to the config layer and `runtime.toml` to
    /// the runtime layer, then starts hot-reload file watching.
    ///
    /// # Errors
    ///
    /// Returns error if config files cannot be loaded or parsed.
    #[instrument]
    pub async fn load() -> Result<Arc<Self>, Error> {
        info!("Loading configuration");

        let config = Config::default();
        let config_path = ConfigPaths::main_config();

        match Self::load_toml_file(&config_path) {
            Ok(config_toml) => config.apply_config_layer(&config_toml, ""),
            Err(e) => warn!(error = %e, "cannot load config.toml, using defaults"),
        }

        let runtime_path = ConfigPaths::runtime_config();
        match Self::load_toml_file(&runtime_path) {
            Ok(runtime_toml) => config.apply_runtime_layer(&runtime_toml, ""),
            Err(e) => warn!(error = %e, "cannot load runtime.toml"),
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
            .map_err(|_| Error::WatcherPoisoned)? = Some(watcher);

        info!("Configuration loaded successfully");

        Ok(service)
    }

    /// Reference to the config root.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Persists runtime layer values to `runtime.toml`.
    ///
    /// Only values with runtime overrides are written.
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
            toml::to_string_pretty(&runtime_value).map_err(|source| Error::Serialization {
                content_type: "runtime config",
                source,
            })?;

        fs::write(&temp_path, toml_str).map_err(|source| Error::Persistence {
            path: temp_path.clone(),
            source,
        })?;

        fs::rename(&temp_path, &runtime_path).map_err(|source| Error::Persistence {
            path: runtime_path.clone(),
            source,
        })?;

        info!("Configuration saved to runtime.toml");

        Ok(())
    }

    fn load_toml_file(path: &std::path::Path) -> Result<toml::Value, Error> {
        let content = fs::read_to_string(path).map_err(|source| Error::Io {
            operation: IoOperation::ReadFile,
            path: path.to_path_buf(),
            source,
        })?;

        toml::from_str(&content).map_err(|source| Error::TomlParse {
            path: path.to_path_buf(),
            source,
        })
    }
}

/// CLI extension for string-based config access.
///
/// Application code should prefer strongly-typed access via `config()`.
pub trait ConfigServiceCli {
    /// Retrieves value at a dot-separated path (e.g., `battery.enabled`).
    ///
    /// # Errors
    ///
    /// Returns error if path is invalid.
    fn get_by_path(&self, path: &str) -> Result<toml::Value, Error>;

    /// Sets a runtime override at a dot-separated path.
    ///
    /// # Errors
    ///
    /// Returns error if path is invalid.
    fn set_by_path(&self, path: &str, value: toml::Value) -> Result<(), Error>;
}

impl ConfigServiceCli for ConfigService {
    fn get_by_path(&self, path: &str) -> Result<toml::Value, Error> {
        let config_value =
            toml::Value::try_from(self.config.as_ref()).map_err(|source| Error::Serialization {
                content_type: "config",
                source,
            })?;

        let mut value = config_value;
        for segment in path.split('.') {
            value = value
                .get(segment)
                .ok_or_else(|| Error::InvalidConfigField {
                    field: segment.to_string(),
                    component: path.to_string(),
                    reason: InvalidFieldReason::NotFound,
                })?
                .clone();
        }

        Ok(value)
    }

    fn set_by_path(&self, path: &str, value: toml::Value) -> Result<(), Error> {
        let mut root = toml::Value::Table(toml::Table::new());
        toml_path::insert(&mut root, path, value)?;
        self.config.apply_runtime_layer(&root, "");
        Ok(())
    }
}
