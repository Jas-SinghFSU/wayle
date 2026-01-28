use std::{sync::RwLock, time::Duration};

use tokio_util::sync::CancellationToken;
use tracing::instrument;
use wayle_common::Property;

use crate::{
    polling,
    service::SysinfoService,
    types::{CpuData, DiskData, MemoryData, NetworkData},
};

const DEFAULT_CPU_INTERVAL: Duration = Duration::from_secs(2);
const DEFAULT_MEMORY_INTERVAL: Duration = Duration::from_secs(5);
const DEFAULT_DISK_INTERVAL: Duration = Duration::from_secs(30);
const DEFAULT_NETWORK_INTERVAL: Duration = Duration::from_secs(2);

/// Builder for configuring a [`SysinfoService`].
pub struct SysinfoServiceBuilder {
    cpu_interval: Duration,
    memory_interval: Duration,
    disk_interval: Duration,
    network_interval: Duration,
    cpu_temp_sensor: String,
}

impl SysinfoServiceBuilder {
    /// Creates a new builder with default intervals.
    pub fn new() -> Self {
        Self {
            cpu_interval: DEFAULT_CPU_INTERVAL,
            memory_interval: DEFAULT_MEMORY_INTERVAL,
            disk_interval: DEFAULT_DISK_INTERVAL,
            network_interval: DEFAULT_NETWORK_INTERVAL,
            cpu_temp_sensor: String::from("auto"),
        }
    }

    /// Sets the CPU polling interval.
    pub fn cpu_interval(mut self, interval: Duration) -> Self {
        self.cpu_interval = interval;
        self
    }

    /// Sets the memory polling interval.
    pub fn memory_interval(mut self, interval: Duration) -> Self {
        self.memory_interval = interval;
        self
    }

    /// Sets the disk polling interval.
    pub fn disk_interval(mut self, interval: Duration) -> Self {
        self.disk_interval = interval;
        self
    }

    /// Sets the network polling interval.
    pub fn network_interval(mut self, interval: Duration) -> Self {
        self.network_interval = interval;
        self
    }

    /// Sets the CPU temperature sensor label.
    ///
    /// Use `"auto"` for automatic detection, or specify a sensor label
    /// (e.g., `"Tctl"`, `"Package id 0"`, `"coretemp"`).
    pub fn cpu_temp_sensor(mut self, sensor: impl Into<String>) -> Self {
        self.cpu_temp_sensor = sensor.into();
        self
    }

    /// Builds the service and starts background polling tasks.
    #[instrument(skip_all, name = "SysinfoService::build")]
    pub fn build(self) -> SysinfoService {
        let cancellation_token = CancellationToken::new();

        let cpu = Property::new(CpuData::default());
        let memory = Property::new(MemoryData::default());
        let disks = Property::new(Vec::<DiskData>::new());
        let network = Property::new(Vec::<NetworkData>::new());

        let tokens = polling::spawn_polling_tasks(
            &cancellation_token,
            &cpu,
            &memory,
            &disks,
            &network,
            self.cpu_interval,
            self.memory_interval,
            self.disk_interval,
            self.network_interval,
            self.cpu_temp_sensor.clone(),
        );

        SysinfoService {
            cancellation_token,
            cpu_token: RwLock::new(tokens.cpu),
            memory_token: RwLock::new(tokens.memory),
            disk_token: RwLock::new(tokens.disk),
            network_token: RwLock::new(tokens.network),
            cpu_interval: RwLock::new(self.cpu_interval),
            cpu_temp_sensor: RwLock::new(self.cpu_temp_sensor),
            cpu,
            memory,
            disks,
            network,
        }
    }
}

impl Default for SysinfoServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}
