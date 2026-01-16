use std::sync::Arc;

use tokio::sync::mpsc;
use tracing::{debug, error, info, instrument};
use wayle_common::SubscribeChanges;

use super::{error::Error, service::ConfigService};

/// Auto-saves configuration changes to disk.
///
/// Monitors all `ConfigProperty` fields and persists changes when
/// any property is modified.
pub struct PersistenceWatcher;

impl PersistenceWatcher {
    /// Starts watching for config changes and auto-saving to disk.
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
