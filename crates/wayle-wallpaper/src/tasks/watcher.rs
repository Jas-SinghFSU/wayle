//! Filesystem watching for directory changes.

use std::path::{Path, PathBuf};

use notify::{
    Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
    event::{CreateKind, ModifyKind, RemoveKind},
    recommended_watcher,
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, warn};

/// Sender for directory change notifications.
pub type DirectoryChangeSender = UnboundedSender<PathBuf>;

/// Watches a directory for file changes and notifies via channel.
///
/// Detects file creation, deletion, and renames. Sends the directory path
/// (not individual file paths) when changes occur.
pub struct DirectoryWatcher {
    _watcher: RecommendedWatcher,
}

impl DirectoryWatcher {
    /// Creates a watcher for the given directory.
    ///
    /// Returns `None` if the watcher cannot be created or the directory
    /// cannot be watched.
    pub fn new(directory: &Path, sender: DirectoryChangeSender) -> Option<Self> {
        let dir_path = directory.to_path_buf();

        let watcher = recommended_watcher(move |result: Result<Event, _>| {
            let Ok(event) = result else { return };

            let is_file_change = matches!(
                event.kind,
                EventKind::Create(CreateKind::File)
                    | EventKind::Remove(RemoveKind::File)
                    | EventKind::Modify(ModifyKind::Name(_))
            );

            if is_file_change {
                let _ = sender.send(dir_path.clone());
            }
        });

        let mut watcher = match watcher {
            Ok(watcher) => watcher,
            Err(err) => {
                warn!(error = %err, "cannot create directory watcher");
                return None;
            }
        };

        if let Err(err) = watcher.watch(directory, RecursiveMode::NonRecursive) {
            warn!(
                error = %err,
                directory = %directory.display(),
                "cannot watch directory"
            );
            return None;
        }

        info!(directory = %directory.display(), "Watching directory for changes");

        Some(Self { _watcher: watcher })
    }
}
