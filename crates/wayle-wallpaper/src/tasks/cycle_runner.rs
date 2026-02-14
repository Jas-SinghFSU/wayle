use std::{collections::HashMap, path::PathBuf};

use futures::{StreamExt, future::join_all};
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
    types::{CyclingConfig, CyclingMode, MonitorState},
};

pub(crate) struct CyclingTask {
    cycling: Property<Option<CyclingConfig>>,
    monitors: Property<HashMap<String, MonitorState>>,
    transition: Property<TransitionConfig>,
    shared_cycle: Property<bool>,
    engine_active: Property<bool>,

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
        transition: Property<TransitionConfig>,
        shared_cycle: Property<bool>,
        engine_active: Property<bool>,
    ) -> Self {
        let (directory_change_sender, directory_change_receiver) = mpsc::unbounded_channel();

        Self {
            cycling,
            monitors,
            transition,
            shared_cycle,
            engine_active,
            timer: CyclingTimer::new(),
            directory_watcher: None,
            watched_directory: None,
            directory_change_sender,
            directory_change_receiver,
        }
    }

    pub async fn run(mut self, cancellation: CancellationToken) {
        let mut cycling_stream = self.cycling.watch();
        let mut shared_cycle_stream = self.shared_cycle.watch();

        loop {
            select! {
                _ = cancellation.cancelled() => {
                    info!("Cycling task cancelled");
                    return;
                }

                Some(config) = cycling_stream.next() => {
                    self.handle_cycling_change(config).await;
                }

                Some(shared) = shared_cycle_stream.next() => {
                    self.handle_shared_cycle_change(shared).await;
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

                self.render_current(cfg).await;
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

    async fn handle_shared_cycle_change(&self, shared: bool) {
        let Some(config) = self.cycling.get() else {
            return;
        };

        if config.mode != CyclingMode::Shuffle {
            return;
        }

        let image_count = config.image_count();
        if image_count == 0 {
            return;
        }

        let mut monitors = self.monitors.get();

        if shared {
            let Some(target_index) = monitors.values().next().map(|s| s.cycle_index) else {
                return;
            };

            for state in monitors.values_mut() {
                state.cycle_index = target_index;
            }

            info!("Synchronized cycle indices across monitors");
        } else {
            for state in monitors.values_mut() {
                state.cycle_index = rand::random_range(0..image_count);
            }

            info!("Desynchronized cycle indices across monitors");
        }

        for state in monitors.values_mut() {
            if let Some(path) = config.image_at(state.cycle_index) {
                state.wallpaper = Some(path.clone());
            }
        }

        self.monitors.set(monitors.clone());
        self.apply_wallpapers(&config, &monitors).await;
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

        let independent_shuffle = config.mode == CyclingMode::Shuffle && !self.shared_cycle.get();

        let mut monitors = self.monitors.get();
        for state in monitors.values_mut() {
            if independent_shuffle {
                state.cycle_index = rand::random_range(0..image_count);
            } else {
                state.advance(image_count);
            }

            if let Some(path) = config.image_at(state.cycle_index) {
                state.wallpaper = Some(path.clone());
            }
        }
        self.monitors.set(monitors.clone());

        self.apply_wallpapers(&config, &monitors).await;
        self.timer.schedule(config.interval);
    }

    async fn render_current(&self, config: &CyclingConfig) {
        let mut monitors = self.monitors.get();

        for state in monitors.values_mut() {
            if let Some(path) = config.image_at(state.cycle_index) {
                state.wallpaper = Some(path.clone());
            }
        }

        self.monitors.set(monitors.clone());
        self.apply_wallpapers(config, &monitors).await;
    }

    async fn apply_wallpapers(
        &self,
        config: &CyclingConfig,
        monitors: &HashMap<String, MonitorState>,
    ) {
        if !self.engine_active.get() {
            return;
        }

        let transition = self.transition.get();

        let tasks: Vec<_> = monitors
            .iter()
            .filter_map(|(name, state)| {
                config
                    .image_at(state.cycle_index)
                    .map(|path| (name, path, state.fit_mode))
            })
            .collect();

        let results = join_all(tasks.iter().map(|(name, path, fit_mode)| {
            SwwwBackend::apply(path, *fit_mode, Some(name.as_str()), &transition)
        }))
        .await;

        for (result, (name, path, _)) in results.into_iter().zip(tasks.iter()) {
            if let Err(err) = result {
                warn!(
                    error = %err,
                    monitor = %name,
                    path = %path.display(),
                    "cannot apply wallpaper"
                );
            }
        }
    }
}
