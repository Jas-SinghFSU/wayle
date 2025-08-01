use std::sync::{Arc, Weak};

use futures::StreamExt;
use tracing::debug;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::services::network_manager::{
    ConnectionActiveProxy, NMActivationStateFlags, NMActiveConnectionState,
};

use super::ActiveConnection;

/// Monitors D-Bus properties and updates the reactive ActiveConnection model.
pub(crate) struct ActiveConnectionMonitor;

impl ActiveConnectionMonitor {
    /// Start monitoring for property changes.
    ///
    /// Monitoring stops automatically when the ActiveConnection is dropped.
    pub async fn start(
        active_connection: Arc<ActiveConnection>,
        zbus_connection: Connection,
        path: OwnedObjectPath,
    ) {
        let weak = Arc::downgrade(&active_connection);

        let Ok(proxy) = ConnectionActiveProxy::new(&zbus_connection, path).await else {
            debug!("Failed to create proxy for active connection monitoring");
            return;
        };

        tokio::spawn(async move {
            Self::monitor(weak, proxy).await;
        });
    }

    #[allow(clippy::cognitive_complexity)]
    #[allow(clippy::too_many_lines)]
    async fn monitor(weak: Weak<ActiveConnection>, proxy: ConnectionActiveProxy<'static>) {
        let mut connection_changes = proxy.receive_connection_changed().await;
        let mut specific_object_changes = proxy.receive_specific_object_changed().await;
        let mut id_changes = proxy.receive_id_changed().await;
        let mut uuid_changed = proxy.receive_uuid_changed().await;
        let mut type_changed = proxy.receive_type__changed().await;
        let mut devices_changed = proxy.receive_devices_changed().await;
        let mut state_changed = proxy.receive_state_changed().await;
        let mut state_flags_changed = proxy.receive_state_flags_changed().await;
        let mut default_changed = proxy.receive_default_changed().await;
        let mut ip4_config_changed = proxy.receive_ip4_config_changed().await;
        let mut dhcp4_config_changed = proxy.receive_dhcp4_config_changed().await;
        let mut default6_changed = proxy.receive_default6_changed().await;
        let mut ip6_config_changed = proxy.receive_ip6_config_changed().await;
        let mut dhcp6_config_changed = proxy.receive_dhcp6_config_changed().await;
        let mut vpn_changed = proxy.receive_vpn_changed().await;
        let mut controller_changed = proxy.receive_controller_changed().await;

        loop {
            let Some(active_connection) = weak.upgrade() else {
                debug!("ActiveConnection dropped, stopping monitor");
                return;
            };

            tokio::select! {
                Some(change) = connection_changes.next() => {
                    if let Ok(new_connection) = change.get().await {
                        active_connection.connection_path.set(new_connection);
                    }
                }
                Some(change) = specific_object_changes.next() => {
                    if let Ok(new_specific_object) = change.get().await {
                        active_connection.specific_object.set(new_specific_object);
                    }
                }
                Some(change) = id_changes.next() => {
                    if let Ok(new_id) = change.get().await {
                        active_connection.id.set(new_id);
                    }
                }
                Some(change) = uuid_changed.next() => {
                    if let Ok(new_uuid) = change.get().await {
                        active_connection.uuid.set(new_uuid);
                    }
                }
                Some(change) = type_changed.next() => {
                    if let Ok(new_type) = change.get().await {
                        active_connection.type_.set(new_type);
                    }
                }
                Some(change) = devices_changed.next() => {
                    if let Ok(new_devices) = change.get().await {
                        active_connection.devices.set(new_devices);
                    }
                }
                Some(change) = state_changed.next() => {
                    if let Ok(new_state) = change.get().await {
                        let state = NMActiveConnectionState::from_u32(new_state);
                        active_connection.state.set(state);
                    }
                }
                Some(change) = state_flags_changed.next() => {
                    if let Ok(new_flags) = change.get().await {
                        let flags = NMActivationStateFlags::from_bits_truncate(new_flags);
                        active_connection.state_flags.set(flags);
                    }
                }
                Some(change) = default_changed.next() => {
                    if let Ok(new_default) = change.get().await {
                        active_connection.default.set(new_default);
                    }
                }
                Some(change) = ip4_config_changed.next() => {
                    if let Ok(new_ip4_config) = change.get().await {
                        active_connection.ip4_config.set(new_ip4_config);
                    }
                }
                Some(change) = dhcp4_config_changed.next() => {
                    if let Ok(new_dhcp4_config) = change.get().await {
                        active_connection.dhcp4_config.set(new_dhcp4_config);
                    }
                }
                Some(change) = default6_changed.next() => {
                    if let Ok(new_default6) = change.get().await {
                        active_connection.default6.set(new_default6);
                    }
                }
                Some(change) = ip6_config_changed.next() => {
                    if let Ok(new_ip6_config) = change.get().await {
                        active_connection.ip6_config.set(new_ip6_config);
                    }
                }
                Some(change) = dhcp6_config_changed.next() => {
                    if let Ok(new_dhcp6_config) = change.get().await {
                        active_connection.dhcp6_config.set(new_dhcp6_config);
                    }
                }
                Some(change) = vpn_changed.next() => {
                    if let Ok(new_vpn) = change.get().await {
                        active_connection.vpn.set(new_vpn);
                    }
                }
                Some(change) = controller_changed.next() => {
                    if let Ok(new_controller) = change.get().await {
                        active_connection.controller.set(new_controller);
                    }
                }
                else => {
                    debug!("All property streams ended for active connection");
                    break;
                }
            }

            drop(active_connection);
        }

        debug!("Property monitoring ended for active connection");
    }
}
