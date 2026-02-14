//! Wallpaper service hot-reload watcher.

use std::{path::PathBuf, sync::Arc, time::Duration};

use futures::StreamExt;
use tracing::warn;
use wayle_config::schemas::wallpaper::{MonitorWallpaperConfig, WallpaperConfig};
use wayle_wallpaper::{TransitionConfig, TransitionDuration, TransitionFps, WallpaperService};

use crate::{shell::ShellServices, wallpaper_map};

pub(crate) fn spawn(services: &ShellServices) {
    let Some(wallpaper) = services.wallpaper.clone() else {
        return;
    };

    let config = services.config.config().wallpaper.clone();

    spawn_transition_watcher(&config, &wallpaper);
    spawn_cycling_watcher(&config, &wallpaper);
    spawn_cycling_interval_watcher(&config, &wallpaper);
    spawn_shared_cycle_watcher(&config, &wallpaper);
    spawn_engine_watcher(&config, &wallpaper);
    spawn_monitors_watcher(&config, &wallpaper);
}

fn spawn_transition_watcher(config: &WallpaperConfig, wallpaper: &Arc<WallpaperService>) {
    let transition_type = config.transition_type.clone();
    let transition_duration = config.transition_duration.clone();
    let transition_fps = config.transition_fps.clone();
    let wallpaper = wallpaper.clone();

    let mut type_stream = transition_type.watch();
    let mut duration_stream = transition_duration.watch();
    let mut fps_stream = transition_fps.watch();

    tokio::spawn(async move {
        type_stream.next().await;
        duration_stream.next().await;
        fps_stream.next().await;

        loop {
            tokio::select! {
                Some(_) = type_stream.next() => {}
                Some(_) = duration_stream.next() => {}
                Some(_) = fps_stream.next() => {}
                else => break,
            }

            let transition = TransitionConfig {
                transition_type: wallpaper_map::transition_type(transition_type.get()),
                duration: TransitionDuration(transition_duration.get().value()),
                fps: TransitionFps(transition_fps.get().value()),
                step: None,
            };
            wallpaper.set_transition(transition);
        }
    });
}

fn spawn_cycling_watcher(config: &WallpaperConfig, wallpaper: &Arc<WallpaperService>) {
    let cycling_enabled = config.cycling_enabled.clone();
    let cycling_directory = config.cycling_directory.clone();
    let cycling_mode = config.cycling_mode.clone();
    let cycling_interval = config.cycling_interval_mins.clone();
    let monitors_config = config.monitors.clone();
    let wallpaper = wallpaper.clone();

    let mut enabled_stream = cycling_enabled.watch();
    let mut directory_stream = cycling_directory.watch();
    let mut mode_stream = cycling_mode.watch();

    tokio::spawn(async move {
        enabled_stream.next().await;
        directory_stream.next().await;
        mode_stream.next().await;

        loop {
            tokio::select! {
                Some(_) = enabled_stream.next() => {}
                Some(_) = directory_stream.next() => {}
                Some(_) = mode_stream.next() => {}
                else => break,
            }

            if !cycling_enabled.get() {
                wallpaper.stop_cycling();
                restore_monitor_wallpapers(&wallpaper, &monitors_config.get()).await;
                continue;
            }

            let directory = cycling_directory.get();
            if directory.is_empty() {
                continue;
            }

            let mode = wallpaper_map::cycling_mode(cycling_mode.get());
            let interval = Duration::from_secs(cycling_interval.get().value() * 60);

            if let Err(e) = wallpaper.start_cycling(PathBuf::from(directory), interval, mode) {
                warn!(error = %e, "could not apply cycling config change");
            }
        }
    });
}

fn spawn_cycling_interval_watcher(config: &WallpaperConfig, wallpaper: &Arc<WallpaperService>) {
    let mut stream = config.cycling_interval_mins.watch();
    let wallpaper = wallpaper.clone();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(interval) = stream.next().await {
            wallpaper.set_cycling_interval(Duration::from_secs(interval.value() * 60));
        }
    });
}

fn spawn_shared_cycle_watcher(config: &WallpaperConfig, wallpaper: &Arc<WallpaperService>) {
    let mut stream = config.cycling_same_image.watch();
    let wallpaper = wallpaper.clone();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(shared) = stream.next().await {
            wallpaper.shared_cycle.set(shared);
        }
    });
}

fn spawn_engine_watcher(config: &WallpaperConfig, wallpaper: &Arc<WallpaperService>) {
    let mut stream = config.engine_enabled.watch();
    let wallpaper = wallpaper.clone();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(enabled) = stream.next().await {
            wallpaper.engine_active.set(enabled);
        }
    });
}

fn spawn_monitors_watcher(config: &WallpaperConfig, wallpaper: &Arc<WallpaperService>) {
    let mut stream = config.monitors.watch();
    let wallpaper = wallpaper.clone();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(monitor_configs) = stream.next().await {
            for monitor_cfg in &monitor_configs {
                apply_monitor_config_change(&wallpaper, monitor_cfg).await;
            }
        }
    });
}

async fn apply_monitor_config_change(
    wallpaper: &WallpaperService,
    monitor_cfg: &MonitorWallpaperConfig,
) {
    if monitor_cfg.name.is_empty() {
        return;
    }

    let fit_mode = wallpaper_map::fit_mode(monitor_cfg.fit_mode);

    if let Err(e) = wallpaper
        .set_fit_mode(fit_mode, Some(&monitor_cfg.name))
        .await
    {
        warn!(
            error = %e,
            monitor = %monitor_cfg.name,
            "could not apply fit mode from config change"
        );
    }

    if monitor_cfg.wallpaper.is_empty() {
        return;
    }

    let path = PathBuf::from(&monitor_cfg.wallpaper);
    if let Err(e) = wallpaper.set_wallpaper(path, Some(&monitor_cfg.name)).await {
        warn!(
            error = %e,
            monitor = %monitor_cfg.name,
            "could not apply wallpaper from config change"
        );
    }
}

async fn restore_monitor_wallpapers(
    wallpaper: &WallpaperService,
    monitors: &[MonitorWallpaperConfig],
) {
    for monitor_cfg in monitors {
        if monitor_cfg.name.is_empty() || monitor_cfg.wallpaper.is_empty() {
            continue;
        }

        let path = PathBuf::from(&monitor_cfg.wallpaper);
        if let Err(e) = wallpaper.set_wallpaper(path, Some(&monitor_cfg.name)).await {
            warn!(
                error = %e,
                monitor = %monitor_cfg.name,
                "cannot restore monitor wallpaper"
            );
        }
    }
}
