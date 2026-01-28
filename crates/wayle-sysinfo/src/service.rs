use std::{sync::RwLock, time::Duration};

use tokio_util::sync::CancellationToken;
use tracing::debug;
use wayle_common::Property;

use crate::{
    builder::SysinfoServiceBuilder,
    polling,
    types::{CpuData, DiskData, MemoryData, NetworkData},
};

/// System information service for monitoring CPU, memory, disk, and network.
///
/// Provides reactive properties that update at configurable intervals.
/// All metrics are polled in the background and exposed via `Property<T>`
/// for both snapshot access (`.get()`) and stream-based watching (`.watch()`).
///
/// Polling intervals can be changed at runtime via `set_*_interval()` methods.
#[derive(Debug)]
pub struct SysinfoService {
    pub(crate) cancellation_token: CancellationToken,
    pub(crate) cpu_token: RwLock<CancellationToken>,
    pub(crate) memory_token: RwLock<CancellationToken>,
    pub(crate) disk_token: RwLock<CancellationToken>,
    pub(crate) network_token: RwLock<CancellationToken>,
    pub(crate) cpu_interval: RwLock<Duration>,
    pub(crate) cpu_temp_sensor: RwLock<String>,

    /// CPU metrics including usage, frequency, and temperature.
    pub cpu: Property<CpuData>,

    /// Memory and swap metrics.
    pub memory: Property<MemoryData>,

    /// Disk metrics for all mounted filesystems.
    pub disks: Property<Vec<DiskData>>,

    /// Network metrics for all interfaces.
    pub network: Property<Vec<NetworkData>>,
}

impl SysinfoService {
    /// Returns a builder for configuring the service.
    pub fn builder() -> SysinfoServiceBuilder {
        SysinfoServiceBuilder::new()
    }

    /// Updates the CPU polling interval.
    pub fn set_cpu_interval(&self, interval: Duration) {
        debug!(?interval, "Updating CPU polling interval");
        if let Ok(mut guard) = self.cpu_interval.write() {
            *guard = interval;
        }
        self.restart_cpu_polling();
    }

    /// Updates the CPU temperature sensor label.
    pub fn set_cpu_temp_sensor(&self, sensor: &str) {
        debug!(?sensor, "Updating CPU temperature sensor");
        if let Ok(mut guard) = self.cpu_temp_sensor.write() {
            *guard = sensor.to_owned();
        }
        self.restart_cpu_polling();
    }

    fn restart_cpu_polling(&self) {
        let interval = self.cpu_interval.read().map(|g| *g).unwrap_or_default();
        let sensor = self
            .cpu_temp_sensor
            .read()
            .map(|g| g.clone())
            .unwrap_or_default();

        let new_token = self.cancellation_token.child_token();
        if let Ok(mut guard) = self.cpu_token.write() {
            guard.cancel();
            polling::cpu::spawn(new_token.clone(), self.cpu.clone(), interval, sensor);
            *guard = new_token;
        }
    }

    /// Updates the memory polling interval.
    ///
    /// Restarts the memory polling task with the new interval.
    pub fn set_memory_interval(&self, interval: Duration) {
        debug!(?interval, "Updating memory polling interval");
        let new_token = self.cancellation_token.child_token();
        if let Ok(mut guard) = self.memory_token.write() {
            guard.cancel();
            polling::memory::spawn(new_token.clone(), self.memory.clone(), interval);
            *guard = new_token;
        }
    }

    /// Updates the disk polling interval.
    ///
    /// Restarts the disk polling task with the new interval.
    pub fn set_disk_interval(&self, interval: Duration) {
        debug!(?interval, "Updating disk polling interval");
        let new_token = self.cancellation_token.child_token();
        if let Ok(mut guard) = self.disk_token.write() {
            guard.cancel();
            polling::disk::spawn(new_token.clone(), self.disks.clone(), interval);
            *guard = new_token;
        }
    }

    /// Updates the network polling interval.
    ///
    /// Restarts the network polling task with the new interval.
    pub fn set_network_interval(&self, interval: Duration) {
        debug!(?interval, "Updating network polling interval");
        let new_token = self.cancellation_token.child_token();
        if let Ok(mut guard) = self.network_token.write() {
            guard.cancel();
            polling::network::spawn(new_token.clone(), self.network.clone(), interval);
            *guard = new_token;
        }
    }
}

impl Drop for SysinfoService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
