mod cycle_runner;
mod timer;
mod watcher;

use std::sync::Arc;

use cycle_runner::CyclingTask;
use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use wayle_traits::ServiceMonitoring;

use crate::{
    error::Error,
    service::WallpaperService,
    wayland::{OutputEvent, OutputWatcher},
};

impl ServiceMonitoring for WallpaperService {
    type Error = Error;

    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        discover_initial_outputs(self);
        spawn_cycling_task(self);
        Ok(())
    }
}

fn discover_initial_outputs(service: &WallpaperService) {
    let Some(outputs) = OutputWatcher::query_outputs() else {
        warn!("cannot query wayland outputs, no monitors registered");
        return;
    };

    if outputs.is_empty() {
        warn!("No Wayland outputs found");
        return;
    }

    info!(count = outputs.len(), "Discovered Wayland outputs");
    for output in outputs {
        service.register_monitor(&output);
    }
}

fn spawn_cycling_task(service: &WallpaperService) {
    let task = CyclingTask::new(
        service.cycling.clone(),
        service.monitors.clone(),
        service.fit_mode.clone(),
        service.transition.clone(),
    );

    let cancellation = service.cancellation_token.clone();

    tokio::spawn(async move {
        task.run(cancellation).await;
    });
}

pub(crate) fn spawn_output_watcher(service: Arc<WallpaperService>) {
    let Some(mut watcher) = OutputWatcher::start() else {
        return;
    };

    let cancellation = service.cancellation_token.clone();

    tokio::spawn(async move {
        run_output_watcher(&mut watcher, &service, cancellation).await;
    });
}

async fn run_output_watcher(
    watcher: &mut OutputWatcher,
    service: &WallpaperService,
    cancellation: CancellationToken,
) {
    loop {
        tokio::select! {
            _ = cancellation.cancelled() => {
                info!("Output watcher cancelled");
                return;
            }

            Some(event) = watcher.events().recv() => {
                match event {
                    OutputEvent::Added(name) => {
                        service.register_monitor(&name);
                    }
                    OutputEvent::Removed(name) => {
                        service.unregister_monitor(&name);
                    }
                }
            }
        }
    }
}

pub(crate) fn spawn_color_extractor(service: Arc<WallpaperService>) {
    let cancellation = service.cancellation_token.clone();

    tokio::spawn(async move {
        let mut monitor_watch = service.monitors.watch();
        let mut color_extractor = service.color_extractor.watch();

        loop {
            tokio::select! {
                _ = cancellation.cancelled() => {
                    info!("Color extractor cancelled");
                    return;
                }

                _ = monitor_watch.next() => {
                    if let Err(e) = service.extract_colors().await {
                        error!(error = %e, "cannot extract colors");
                    }
                }

                _ = color_extractor.next() => {
                    service.last_extracted_wallpaper.set(None);
                    if let Err(e) = service.extract_colors().await {
                        error!(error = %e, "cannot extract colors");
                    }
                }
            }
        }
    });
}
