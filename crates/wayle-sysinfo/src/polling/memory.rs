use std::time::Duration;

use sysinfo::System;
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use wayle_common::Property;

use crate::types::MemoryData;

pub(crate) fn spawn(
    token: CancellationToken,
    memory: Property<MemoryData>,
    poll_interval: Duration,
) {
    tokio::spawn(async move {
        let mut system = System::new();
        let mut ticker = interval(poll_interval);

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    debug!("Memory polling cancelled");
                    return;
                }
                _ = ticker.tick() => {
                    system.refresh_memory();

                    let total = system.total_memory();
                    let used = system.used_memory();
                    let available = system.available_memory();
                    let swap_total = system.total_swap();
                    let swap_used = system.used_swap();

                    let usage_percent = if total > 0 {
                        (used as f32 / total as f32) * 100.0
                    } else {
                        0.0
                    };

                    let swap_percent = if swap_total > 0 {
                        (swap_used as f32 / swap_total as f32) * 100.0
                    } else {
                        0.0
                    };

                    memory.set(MemoryData {
                        total_bytes: total,
                        used_bytes: used,
                        available_bytes: available,
                        usage_percent,
                        swap_total_bytes: swap_total,
                        swap_used_bytes: swap_used,
                        swap_percent,
                    });
                }
            }
        }
    });
}
