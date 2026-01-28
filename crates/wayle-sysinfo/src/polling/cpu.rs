use std::time::Duration;

use sysinfo::{Components, System};
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use wayle_common::Property;

use crate::types::{CoreData, CpuData};

const CPU_TEMP_PATTERNS: &[&str] = &[
    "tctl",       // AMD Ryzen primary temp
    "tdie",       // AMD die temp
    "tccd",       // AMD CCD temp
    "k10temp",    // AMD sensor name
    "coretemp",   // Intel sensor name
    "package id", // Intel package temp
    "cpu",        // Generic fallback
];

fn find_cpu_temperature(components: &Components, sensor: &str) -> Option<f32> {
    if sensor != "auto" {
        return components
            .iter()
            .find(|c| c.label().to_lowercase().contains(&sensor.to_lowercase()))
            .and_then(|c| c.temperature());
    }

    for pattern in CPU_TEMP_PATTERNS {
        if let Some(temp) = components
            .iter()
            .find(|c| c.label().to_lowercase().contains(pattern))
            .and_then(|c| c.temperature())
        {
            return Some(temp);
        }
    }
    None
}

pub(crate) fn spawn(
    token: CancellationToken,
    cpu: Property<CpuData>,
    poll_interval: Duration,
    temp_sensor: String,
) {
    tokio::spawn(async move {
        let mut system = System::new();
        let mut components = Components::new_with_refreshed_list();
        let mut ticker = interval(poll_interval);

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    debug!("CPU polling cancelled");
                    return;
                }
                _ = ticker.tick() => {
                    system.refresh_cpu_all();
                    components.refresh(false);

                    let cores: Vec<CoreData> = system
                        .cpus()
                        .iter()
                        .map(|c| CoreData {
                            name: c.name().to_string(),
                            usage_percent: c.cpu_usage(),
                            frequency_mhz: c.frequency(),
                        })
                        .collect();

                    let (avg_freq, max_freq, busiest_freq) = if cores.is_empty() {
                        (0, 0, 0)
                    } else {
                        let sum: u64 = cores.iter().map(|c| c.frequency_mhz).sum();
                        let max = cores.iter().map(|c| c.frequency_mhz).max().unwrap_or(0);
                        let busiest = cores
                            .iter()
                            .max_by(|a, b| {
                                a.usage_percent
                                    .partial_cmp(&b.usage_percent)
                                    .unwrap_or(std::cmp::Ordering::Equal)
                            })
                            .map(|c| c.frequency_mhz)
                            .unwrap_or(0);
                        (sum / cores.len() as u64, max, busiest)
                    };

                    let temperature = find_cpu_temperature(&components, &temp_sensor);

                    cpu.set(CpuData {
                        usage_percent: system.global_cpu_usage(),
                        avg_frequency_mhz: avg_freq,
                        max_frequency_mhz: max_freq,
                        busiest_core_freq_mhz: busiest_freq,
                        temperature_celsius: temperature,
                        cores,
                    });
                }
            }
        }
    });
}
