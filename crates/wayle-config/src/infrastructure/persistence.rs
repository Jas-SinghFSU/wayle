use std::sync::Arc;

use tokio::sync::mpsc;
use tracing::{debug, error, info, instrument};
use wayle_common::SubscribeChanges;

use super::{error::Error, service::ConfigService};

/// Watches for configuration changes and automatically persists to disk.
///
/// Spawns background tasks that monitor all Properties in the config tree.
/// When any Property changes, the entire config is serialized and saved.
pub struct PersistenceWatcher;

impl PersistenceWatcher {
    /// Start watching for config changes and auto-saving to disk.
    ///
    /// # Arguments
    ///
    /// * `config_service` - The config service to watch and persist
    ///
    /// # Errors
    ///
    /// Returns error if persistence cannot be initialized.
    #[instrument(skip(config_service))]
    pub fn start(config_service: Arc<ConfigService>) -> Result<Self, Error> {
        let (tx, mut rx) = mpsc::unbounded_channel();

        config_service.config().subscribe_changes(tx);

        info!("Config persistence watcher started");

        let service = Arc::clone(&config_service);
        tokio::spawn(async move {
            while rx.recv().await.is_some() {
                debug!("Config change detected, persisting to disk");

                if let Err(e) = service.save().await {
                    error!(error = %e, "cannot persist config changes");
                }
            }
        });

        Ok(Self)
    }
}
