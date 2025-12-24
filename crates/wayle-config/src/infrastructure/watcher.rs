use std::{path::PathBuf, sync::Arc};

use notify::{
    Event, RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher, event::EventKind,
};
use tokio::sync::mpsc;
use tracing::{debug, error, info, instrument};
use wayle_common::ApplyConfigLayer;

use super::{error::Error, paths::ConfigPaths, service::ConfigService};
use crate::{Config, infrastructure::themes::utils::load_themes};

/// Watches configuration files for changes and syncs to Properties.
///
/// When config files are modified on disk, the watcher automatically
/// reloads them and updates the corresponding Property values.
/// Only changed Properties are updated via send_if_modified, preventing circular writes.
#[derive(Clone)]
pub struct FileWatcher {
    config_service: Arc<ConfigService>,
    _watcher: Arc<RecommendedWatcher>,
}

impl FileWatcher {
    /// Start watching config files for changes.
    ///
    /// # Arguments
    ///
    /// * `config_service` - The config service containing Properties to update
    ///
    /// # Errors
    ///
    /// Returns error if file watching cannot be initialized.
    #[instrument(skip(config_service))]
    pub fn start(config_service: Arc<ConfigService>) -> Result<Self, Error> {
        let (tx, mut rx) = mpsc::unbounded_channel();

        let mut watcher = notify::recommended_watcher(move |result: Result<Event, _>| {
            if let Ok(event) = result {
                let _ = tx.send(event);
            }
        })
        .map_err(|e| Error::IoError {
            path: PathBuf::from("file watcher"),
            details: e.to_string(),
        })?;

        let config_dir = ConfigPaths::config_dir().map_err(|e| Error::IoError {
            path: PathBuf::from("config directory"),
            details: e.to_string(),
        })?;

        watcher
            .watch(&config_dir, RecursiveMode::Recursive)
            .map_err(|e| Error::IoError {
                path: config_dir.clone(),
                details: e.to_string(),
            })?;

        info!(?config_dir, "Config directory watcher started");

        let file_watcher = Self {
            config_service,
            _watcher: Arc::new(watcher),
        };

        let watcher_clone = file_watcher.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                debug!(?event, "File watcher received event");

                if !Self::should_reload(&event) {
                    continue;
                }

                if let Err(e) = watcher_clone.reload_and_sync(&event.paths).await {
                    error!(error = ?e, "Failed to reload config after file change");
                }
            }
        });

        Ok(file_watcher)
    }

    fn should_reload(event: &Event) -> bool {
        matches!(
            event.kind,
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
        )
    }

    #[instrument(skip(self))]
    async fn reload_and_sync(&self, paths: &[PathBuf]) -> Result<(), Error> {
        let themes_dir = ConfigPaths::themes_dir();
        if paths.iter().any(|path| path.starts_with(&themes_dir)) {
            load_themes(self.config_service.config(), &themes_dir);
            return Ok(());
        }

        let config_path = ConfigPaths::main_config();
        let reloaded_config = Config::load_with_imports(&config_path)?;

        let toml_value: toml::Value =
            toml::Value::try_from(&reloaded_config).map_err(|e| Error::SerializationError {
                content_type: String::from("config"),
                details: e.to_string(),
            })?;

        self.config_service.config().apply_config_layer(&toml_value);

        Ok(())
    }
}
