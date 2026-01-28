use std::time::Duration;

use sysinfo::Disks;
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use wayle_common::Property;

use crate::types::DiskData;

pub(crate) fn spawn(
    token: CancellationToken,
    disks: Property<Vec<DiskData>>,
    poll_interval: Duration,
) {
    tokio::spawn(async move {
        let mut sysinfo_disks = Disks::new_with_refreshed_list();
        let mut ticker = interval(poll_interval);

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    debug!("Disk polling cancelled");
                    return;
                }
                _ = ticker.tick() => {
                    sysinfo_disks.refresh(false);

                    let data: Vec<DiskData> = sysinfo_disks
                        .iter()
                        .map(|disk| {
                            let total = disk.total_space();
                            let available = disk.available_space();
                            let used = total.saturating_sub(available);

                            let usage_percent = if total > 0 {
                                (used as f32 / total as f32) * 100.0
                            } else {
                                0.0
                            };

                            DiskData {
                                mount_point: disk.mount_point().to_path_buf(),
                                filesystem: disk.file_system().to_string_lossy().to_string(),
                                total_bytes: total,
                                used_bytes: used,
                                available_bytes: available,
                                usage_percent,
                            }
                        })
                        .collect();

                    disks.set(data);
                }
            }
        }
    });
}
