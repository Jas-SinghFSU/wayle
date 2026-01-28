//! Sysinfo polling interval hot-reload watcher.

use std::time::Duration;

use futures::StreamExt;
use wayle_common::services;
use wayle_config::ConfigService;
use wayle_sysinfo::SysinfoService;

/// Spawns watchers for sysinfo polling interval configuration.
///
/// Updates the service's polling intervals when config properties change.
pub fn spawn() {
    let config = services::get::<ConfigService>().config().clone();
    let modules = &config.modules;

    spawn_cpu_watcher(&modules.cpu);
    spawn_memory_watcher(&modules.ram);
    spawn_disk_watcher(&modules.storage);
    spawn_network_watcher(&modules.netstat);
}

fn spawn_cpu_watcher(config: &wayle_config::schemas::modules::CpuConfig) {
    let mut stream = config.poll_interval_ms.watch();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(interval_ms) = stream.next().await {
            let sysinfo = services::get::<SysinfoService>();
            sysinfo.set_cpu_interval(Duration::from_millis(interval_ms));
        }
    });
}

fn spawn_memory_watcher(config: &wayle_config::schemas::modules::RamConfig) {
    let mut stream = config.poll_interval_ms.watch();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(interval_ms) = stream.next().await {
            let sysinfo = services::get::<SysinfoService>();
            sysinfo.set_memory_interval(Duration::from_millis(interval_ms));
        }
    });
}

fn spawn_disk_watcher(config: &wayle_config::schemas::modules::StorageConfig) {
    let mut stream = config.poll_interval_ms.watch();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(interval_ms) = stream.next().await {
            let sysinfo = services::get::<SysinfoService>();
            sysinfo.set_disk_interval(Duration::from_millis(interval_ms));
        }
    });
}

fn spawn_network_watcher(config: &wayle_config::schemas::modules::NetstatConfig) {
    let mut stream = config.poll_interval_ms.watch();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(interval_ms) = stream.next().await {
            let sysinfo = services::get::<SysinfoService>();
            sysinfo.set_network_interval(Duration::from_millis(interval_ms));
        }
    });
}
