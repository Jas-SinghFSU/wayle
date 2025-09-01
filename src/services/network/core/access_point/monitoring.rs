use std::sync::Arc;

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use super::{AccessPoint, BSSID, SSID, SecurityType};
use crate::services::{
    network::{
        AccessPointProxy, NM80211ApFlags, NM80211ApSecurityFlags, NM80211Mode, NetworkError,
    },
    traits::ModelMonitoring,
};

impl ModelMonitoring for AccessPoint {
    type Error = NetworkError;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let proxy = AccessPointProxy::new(&self.connection, self.object_path.clone())
            .await
            .map_err(NetworkError::DbusError)?;

        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(NetworkError::OperationFailed {
                operation: "start_monitoring",
                reason: "A cancellation_token was not found.".to_string(),
            });
        };

        let cancel_token = cancellation_token.clone();

        tokio::spawn(async move {
            monitor(self, proxy, cancel_token).await;
        });

        Ok(())
    }
}

#[allow(clippy::cognitive_complexity)]
#[allow(clippy::too_many_lines)]
async fn monitor(
    access_point: Arc<AccessPoint>,
    proxy: AccessPointProxy<'static>,
    cancellation_token: CancellationToken,
) {
    let mut flag_changes = proxy.receive_flags_changed().await;
    let mut wpa_flags_changes = proxy.receive_wpa_flags_changed().await;
    let mut rsn_flags_changes = proxy.receive_rsn_flags_changed().await;
    let mut ssid_changes = proxy.receive_ssid_changed().await;
    let mut frequency_changes = proxy.receive_frequency_changed().await;
    let mut hw_address_changes = proxy.receive_hw_address_changed().await;
    let mut mode_changes = proxy.receive_mode_changed().await;
    let mut max_bitrate_changes = proxy.receive_max_bitrate_changed().await;
    let mut strength_changes = proxy.receive_strength_changed().await;
    let mut last_seen_changes = proxy.receive_last_seen_changed().await;

    loop {
        tokio::select! {
            _ = cancellation_token.cancelled() => {
                debug!("AccessPointMonitor cancelled");
                return;
            }
            Some(change) = flag_changes.next() => {
                if let Ok(new_flags) = change.get().await {
                    let flags = NM80211ApFlags::from_bits_truncate(new_flags);
                    access_point.flags.set(flags);

                    let security = SecurityType::from_flags(
                        flags,
                        access_point.wpa_flags.get(),
                        access_point.rsn_flags.get(),
                    );
                    access_point.security.set(security);
                }
            }
            Some(change) = wpa_flags_changes.next() => {
                if let Ok(new_flags) = change.get().await {
                    let wpa_flags = NM80211ApSecurityFlags::from_bits_truncate(new_flags);
                    access_point.wpa_flags.set(wpa_flags);

                    let security = SecurityType::from_flags(
                        access_point.flags.get(),
                        wpa_flags,
                        access_point.rsn_flags.get(),
                    );
                    access_point.security.set(security);
                }
            }
            Some(change) = rsn_flags_changes.next() => {
                if let Ok(new_flags) = change.get().await {
                    let rsn_flags = NM80211ApSecurityFlags::from_bits_truncate(new_flags);
                    access_point.rsn_flags.set(rsn_flags);

                    let security = SecurityType::from_flags(
                        access_point.flags.get(),
                        access_point.wpa_flags.get(),
                        rsn_flags,
                    );
                    access_point.security.set(security);
                }
            }
            Some(change) = ssid_changes.next() => {
                if let Ok(new_ssid) = change.get().await {
                    let ssid = SSID::new(new_ssid);
                    let is_hidden = ssid.is_empty();
                    access_point.ssid.set(ssid);
                    access_point.is_hidden.set(is_hidden);
                }
            }
            Some(change) = frequency_changes.next() => {
                if let Ok(new_frequency) = change.get().await {
                    access_point.frequency.set(new_frequency);
                }
            }
            Some(change) = hw_address_changes.next() => {
                if let Ok(new_hw_address) = change.get().await {
                    let bssid = BSSID::new(new_hw_address.into_bytes());
                    access_point.bssid.set(bssid);
                }
            }
            Some(change) = mode_changes.next() => {
                if let Ok(new_mode) = change.get().await {
                    let mode = NM80211Mode::from_u32(new_mode);
                    access_point.mode.set(mode);
                }
            }
            Some(change) = max_bitrate_changes.next() => {
                if let Ok(new_bitrate) = change.get().await {
                    access_point.max_bitrate.set(new_bitrate);
                }
            }
            Some(change) = strength_changes.next() => {
                if let Ok(new_strength) = change.get().await {
                    access_point.strength.set(new_strength);
                }
            }
            Some(change) = last_seen_changes.next() => {
                if let Ok(new_last_seen) = change.get().await {
                    access_point.last_seen.set(new_last_seen);
                }
            }
            else => {
                debug!("All property streams ended for access point");
                break;
            }
        }
    }

    debug!("Property monitoring ended for access point");
}
