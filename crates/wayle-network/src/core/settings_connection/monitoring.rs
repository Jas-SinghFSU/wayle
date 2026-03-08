use std::sync::{Arc, Weak};

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use wayle_traits::ModelMonitoring;

use super::{ConnectionSettings, extract_identity};
use crate::{
    error::Error, proxy::settings::connection::SettingsConnectionProxy,
    types::flags::NMConnectionSettingsFlags,
};

impl ModelMonitoring for ConnectionSettings {
    type Error = Error;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let proxy = SettingsConnectionProxy::new(&self.connection, self.object_path.clone())
            .await
            .map_err(Error::DbusError)?;

        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(Error::MissingCancellationToken);
        };

        let cancel_token = cancellation_token.clone();
        let weak_self = Arc::downgrade(&self);

        tokio::spawn(async move {
            monitor(weak_self, proxy, cancel_token).await;
        });

        Ok(())
    }
}

#[allow(clippy::cognitive_complexity)]
async fn monitor(
    weak_settings: Weak<ConnectionSettings>,
    proxy: SettingsConnectionProxy<'static>,
    cancellation_token: CancellationToken,
) {
    let mut unsaved_changed = proxy.receive_unsaved_changed().await;
    let mut flags_changed = proxy.receive_flags_changed().await;
    let mut filename_changed = proxy.receive_filename_changed().await;
    let Ok(mut updated) = proxy.receive_updated().await else {
        debug!("failed to subscribe to Updated signal for SettingsConnection");
        return;
    };

    loop {
        let Some(settings) = weak_settings.upgrade() else {
            return;
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                debug!("ConnectionSettingsMonitor cancelled");
                return;
            }
            Some(change) = unsaved_changed.next() => {
                if let Ok(value) = change.get().await {
                    settings.unsaved.set(value);
                }
            }
            Some(change) = flags_changed.next() => {
                if let Ok(value) = change.get().await {
                    settings.flags.set(NMConnectionSettingsFlags::from_bits_truncate(value));
                }
            }
            Some(change) = filename_changed.next() => {
                if let Ok(value) = change.get().await {
                    settings.filename.set(value);
                }
            }
            Some(_) = updated.next() => {
                if let Ok(settings_map) = proxy.get_settings().await {
                    let (id, uuid, connection_type, wifi_ssid) = extract_identity(&settings_map);
                    settings.id.set(id);
                    settings.uuid.set(uuid);
                    settings.connection_type.set(connection_type);
                    settings.wifi_ssid.set(wifi_ssid);
                }
            }
            else => {
                debug!("All property streams ended for SettingsConnection");
                break;
            }
        }
    }
}
