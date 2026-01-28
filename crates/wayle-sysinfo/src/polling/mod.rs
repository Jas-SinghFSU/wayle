pub(crate) mod cpu;
pub(crate) mod disk;
pub(crate) mod memory;
pub(crate) mod network;

use std::time::Duration;

use tokio_util::sync::CancellationToken;
use wayle_common::Property;

use crate::types::{CpuData, DiskData, MemoryData, NetworkData};

/// Return type for spawning polling tasks, containing the child tokens for each.
pub(crate) struct PollingTokens {
    pub(crate) cpu: CancellationToken,
    pub(crate) memory: CancellationToken,
    pub(crate) disk: CancellationToken,
    pub(crate) network: CancellationToken,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_polling_tasks(
    cancellation_token: &CancellationToken,
    cpu: &Property<CpuData>,
    memory: &Property<MemoryData>,
    disks: &Property<Vec<DiskData>>,
    network: &Property<Vec<NetworkData>>,
    cpu_interval: Duration,
    memory_interval: Duration,
    disk_interval: Duration,
    network_interval: Duration,
    cpu_temp_sensor: String,
) -> PollingTokens {
    let cpu_token = cancellation_token.child_token();
    let memory_token = cancellation_token.child_token();
    let disk_token = cancellation_token.child_token();
    let network_token = cancellation_token.child_token();

    cpu::spawn(
        cpu_token.clone(),
        cpu.clone(),
        cpu_interval,
        cpu_temp_sensor,
    );
    memory::spawn(memory_token.clone(), memory.clone(), memory_interval);
    disk::spawn(disk_token.clone(), disks.clone(), disk_interval);
    network::spawn(network_token.clone(), network.clone(), network_interval);

    PollingTokens {
        cpu: cpu_token,
        memory: memory_token,
        disk: disk_token,
        network: network_token,
    }
}
