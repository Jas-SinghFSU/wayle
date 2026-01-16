use std::{collections::HashMap, path::PathBuf};

use futures::StreamExt;
use tokio::{select, sync::mpsc};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};
use wayle_common::Property;

use super::{
    timer::CyclingTimer,
    watcher::{DirectoryChangeSender, DirectoryWatcher},
};
use crate::{
    backend::{SwwwBackend, TransitionConfig},
    types::{CyclingConfig, FitMode, MonitorState},
};

pub(crate) struct CyclingTask {
    cycling: Property<Option<CyclingConfig>>,
    monitors: Property<HashMap<String, MonitorState>>,
    fit_mode: Property<FitMode>,
    transition: Property<TransitionConfig>,

    timer: CyclingTimer,
    directory_watcher: Option<DirectoryWatcher>,
    watched_directory: Option<PathBuf>,

    directory_change_sender: DirectoryChangeSender,
    directory_change_receiver: mpsc::UnboundedReceiver<PathBuf>,
}

impl CyclingTask {
    pub fn new(
        cycling: Property<Option<CyclingConfig>>,
        monitors: Property<HashMap<String, MonitorState>>,
        fit_mode: Property<FitMode>,
        transition: Property<TransitionConfig>,
    ) -> Self {
        let (directory_change_sender, directory_change_receiver) = mpsc::unbounded_channel();

        Self {
            cycling,
            monitors,
            fit_mode,
            transition,
            timer: CyclingTimer::new(),
            directory_watcher: None,
            watched_directory: None,
            directory_change_sender,
            directory_change_receiver,
        }
    }

    pub async fn run(mut self, cancellation: CancellationToken) {
        let mut cycling_stream = self.cycling.watch();

        loop {
            select! {
                _ = cancellation.cancelled() => {
                    info!("Cycling task cancelled");
                    return;
                }

                Some(config) = cycling_stream.next() => {
                    self.handle_cycling_change(config).await;
                }

                Some(()) = self.timer.wait(), if self.timer.is_scheduled() => {
                    self.handle_timer_fired().await;
                }

                Some(directory) = self.directory_change_receiver.recv() => {
                    self.handle_directory_changed(&directory);
                }
            }
        }
    }

    async fn handle_cycling_change(&mut self, config: Option<CyclingConfig>) {
        match config {
            Some(ref cfg) => {
                info!(
                    directory = %cfg.directory.display(),
                    images = cfg.image_count(),
                    interval_secs = cfg.interval.as_secs(),
                    "Cycling started"
                );

                self.timer.schedule(cfg.interval);
                self.ensure_directory_watcher(&cfg.directory);
            }
            None => {
                if self.timer.is_scheduled() {
                    info!("Cycling stopped");
                }
                self.timer.cancel();
                self.directory_watcher = None;
                self.watched_directory = None;
            }
        }
    }

    fn ensure_directory_watcher(&mut self, directory: &PathBuf) {
        if self.watched_directory.as_ref() == Some(directory) {
            return;
        }

        let sender = self.directory_change_sender.clone();
        self.directory_watcher = DirectoryWatcher::new(directory, sender);
        self.watched_directory = Some(directory.clone());
    }

    fn handle_directory_changed(&mut self, directory: &PathBuf) {
        let Some(watched) = &self.watched_directory else {
            return;
        };

        if watched != directory {
            return;
        }

        let mut cycling = self.cycling.get();
        let Some(ref mut config) = cycling else {
            return;
        };

        if let Err(err) = config.refresh() {
            warn!(error = %err, "cannot refresh cycling images");
            return;
        }

        info!(images = config.image_count(), "Refreshed cycling images");
        self.cycling.set(cycling);
    }

    async fn handle_timer_fired(&mut self) {
        let Some(config) = self.cycling.get() else {
            return;
        };

        let image_count = config.image_count();
        if image_count == 0 {
            return;
        }

        let mut monitors = self.monitors.get();
        for state in monitors.values_mut() {
            state.advance(image_count);
        }
        self.monitors.set(monitors.clone());

        self.apply_wallpapers(&config, &monitors).await;
        self.timer.schedule(config.interval);
    }

    async fn apply_wallpapers(
        &self,
        config: &CyclingConfig,
        monitors: &HashMap<String, MonitorState>,
    ) {
        let fit_mode = self.fit_mode.get();
        let transition = self.transition.get();

        for (monitor_name, state) in monitors {
            let Some(path) = config.image_at(state.cycle_index) else {
                continue;
            };

            if let Err(err) =
                SwwwBackend::apply(path, fit_mode, Some(monitor_name), &transition).await
            {
                warn!(
                    error = %err,
                    monitor = monitor_name,
                    path = %path.display(),
                    "cannot apply wallpaper"
                );
            }
        }
    }
}
