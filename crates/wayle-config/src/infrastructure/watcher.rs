use std::{path::PathBuf, sync::Arc, time::Duration};

use notify::{
    Event, RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher, event::EventKind,
};
use tokio::sync::mpsc;
use tracing::{debug, error, info, instrument};
use wayle_common::{
    ApplyConfigLayer, ApplyRuntimeLayer, CommitConfigReload, ResetConfigLayer, ResetRuntimeLayer,
};

use super::{error::Error, paths::ConfigPaths, service::ConfigService};
use crate::{Config, infrastructure::themes::utils::load_themes};

/// Hot-reloads configuration files on disk changes.
///
/// When config files are modified, reloads them and updates corresponding
/// `ConfigProperty` values. Uses `send_if_modified` to prevent circular writes.
#[derive(Clone)]
pub struct FileWatcher {
    config_service: Arc<ConfigService>,
    _watcher: Arc<RecommendedWatcher>,
}

impl FileWatcher {
    /// Starts watching config directory for changes.
    ///
    /// # Errors
    ///
    /// Returns error if file watching cannot be initialized.
    #[instrument(skip(config_service))]
    pub fn start(config_service: Arc<ConfigService>) -> Result<Self, Error> {
        let (tx, rx) = mpsc::unbounded_channel();

        let mut watcher = notify::recommended_watcher(move |result: Result<Event, _>| {
            if let Ok(event) = result {
                let _ = tx.send(event);
            }
        })
        .map_err(|source| Error::WatcherInit { source })?;

        let config_dir = ConfigPaths::config_dir()?;

        watcher
            .watch(&config_dir, RecursiveMode::Recursive)
            .map_err(|source| Error::Watch {
                path: config_dir.clone(),
                source,
            })?;

        info!(?config_dir, "Config directory watcher started");

        let file_watcher = Self {
            config_service,
            _watcher: Arc::new(watcher),
        };

        tokio::spawn(run_debounced_event_loop(file_watcher.clone(), rx));

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

        let config = self.config_service.config();

        let config_path = ConfigPaths::main_config();
        let toml_value = Config::load_toml_with_imports(&config_path)?;

        config.reset_config_layer();
        config.apply_config_layer(&toml_value, "");

        config.reset_runtime_layer();
        let runtime_path = ConfigPaths::runtime_config();
        if let Ok(runtime_toml) = ConfigService::load_toml_file(&runtime_path) {
            let _ = config.apply_runtime_layer(&runtime_toml, "");
        }

        config.commit_config_reload();

        Ok(())
    }
}

const DEBOUNCE_DURATION: Duration = Duration::from_millis(100);

async fn run_debounced_event_loop(watcher: FileWatcher, mut rx: mpsc::UnboundedReceiver<Event>) {
    use tokio::time::{Instant, sleep_until};

    let mut pending_paths: Vec<PathBuf> = Vec::new();
    let mut deadline: Option<Instant> = None;

    loop {
        let maybe_event = match deadline {
            Some(d) => tokio::select! {
                biased;
                event = rx.recv() => event,
                () = sleep_until(d) => None,
            },
            None => rx.recv().await,
        };

        match maybe_event {
            Some(event) if FileWatcher::should_reload(&event) => {
                accumulate_paths(&mut pending_paths, event.paths);
                deadline = Some(Instant::now() + DEBOUNCE_DURATION);
            }
            Some(_) => {}
            None if deadline.is_some() => {
                flush_pending(&watcher, &mut pending_paths).await;
                deadline = None;
            }
            None => break,
        }
    }
}

fn accumulate_paths(pending: &mut Vec<PathBuf>, new_paths: Vec<PathBuf>) {
    for path in new_paths {
        if !pending.contains(&path) {
            pending.push(path);
        }
    }
}

async fn flush_pending(watcher: &FileWatcher, pending_paths: &mut Vec<PathBuf>) {
    debug!(?pending_paths, "Debounce complete, reloading config");

    if let Err(e) = watcher.reload_and_sync(pending_paths).await {
        error!("config reload failed:\n{e}");
    }

    pending_paths.clear();
}
