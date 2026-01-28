use std::{collections::HashMap, time::Duration};

use sysinfo::Networks;
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use wayle_common::Property;

use crate::types::NetworkData;

pub(crate) fn spawn(
    token: CancellationToken,
    network: Property<Vec<NetworkData>>,
    poll_interval: Duration,
) {
    let interval_secs = poll_interval.as_secs_f64();

    tokio::spawn(async move {
        let mut networks = Networks::new_with_refreshed_list();
        let mut ticker = interval(poll_interval);
        let mut prev_rx: HashMap<String, u64> = HashMap::new();
        let mut prev_tx: HashMap<String, u64> = HashMap::new();

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    debug!("Network polling cancelled");
                    return;
                }
                _ = ticker.tick() => {
                    networks.refresh(true);

                    let data: Vec<NetworkData> = networks
                        .iter()
                        .map(|(name, net)| {
                            let rx = net.total_received();
                            let tx = net.total_transmitted();

                            let last_rx = prev_rx.get(name).copied().unwrap_or(rx);
                            let last_tx = prev_tx.get(name).copied().unwrap_or(tx);

                            let rx_delta = rx.saturating_sub(last_rx);
                            let tx_delta = tx.saturating_sub(last_tx);

                            let rx_per_sec = (rx_delta as f64 / interval_secs) as u64;
                            let tx_per_sec = (tx_delta as f64 / interval_secs) as u64;

                            prev_rx.insert(name.clone(), rx);
                            prev_tx.insert(name.clone(), tx);

                            NetworkData {
                                interface: name.clone(),
                                rx_bytes: rx,
                                tx_bytes: tx,
                                rx_bytes_per_sec: rx_per_sec,
                                tx_bytes_per_sec: tx_per_sec,
                            }
                        })
                        .collect();

                    network.set(data);
                }
            }
        }
    });
}
