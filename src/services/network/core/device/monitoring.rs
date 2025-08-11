use std::sync::{Arc, Weak};

use futures::StreamExt;
use tracing::debug;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::services::network::{
    NMConnectivityState, NMDeviceCapabilities, NMDeviceInterfaceFlags, NMDeviceState,
    NMDeviceStateReason, NMDeviceType, NMMetered, NetworkError, core::device::Device,
    proxy::devices::DeviceProxy,
};

/// Monitors D-Bus properties and updates the reactive Device model.
pub(crate) struct DeviceMonitor;

impl DeviceMonitor {
    pub(super) async fn start(
        device: Arc<Device>,
        connection: &Connection,
        path: OwnedObjectPath,
    ) -> Result<(), NetworkError> {
        let weak = Arc::downgrade(&device);

        let proxy = DeviceProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        tokio::spawn(async move {
            Self::monitor(weak, proxy).await;
        });

        Ok(())
    }

    #[allow(clippy::cognitive_complexity)]
    #[allow(clippy::too_many_lines)]
    async fn monitor(weak: Weak<Device>, proxy: DeviceProxy<'static>) {
        let mut udi_changed = proxy.receive_udi_changed().await;
        let mut udev_path_changed = proxy.receive_path_changed().await;
        let mut interface_changed = proxy.receive_interface_changed().await;
        let mut ip_interface_changed = proxy.receive_ip_interface_changed().await;
        let mut driver_changed = proxy.receive_driver_changed().await;
        let mut driver_version_changed = proxy.receive_driver_version_changed().await;
        let mut firmware_version_changed = proxy.receive_firmware_version_changed().await;
        let mut capabilities_changed = proxy.receive_capabilities_changed().await;
        let mut state_changed = proxy.receive_state_changed().await;
        let mut state_reason_changed = proxy.receive_state_reason_changed().await;
        let mut active_connection_changed = proxy.receive_active_connection_changed().await;
        let mut ip4_config_changed = proxy.receive_ip4_config_changed().await;
        let mut dhcp4_config_changed = proxy.receive_dhcp4_config_changed().await;
        let mut ip6_config_changed = proxy.receive_ip6_config_changed().await;
        let mut dhcp6_config_changed = proxy.receive_dhcp6_config_changed().await;
        let mut managed_changed = proxy.receive_managed_changed().await;
        let mut autoconnect_changed = proxy.receive_autoconnect_changed().await;
        let mut firmware_missing_changed = proxy.receive_firmware_missing_changed().await;
        let mut nm_plugin_missing_changed = proxy.receive_nm_plugin_missing_changed().await;
        let mut device_type_changed = proxy.receive_device_type_changed().await;
        let mut available_connections_changed = proxy.receive_available_connections_changed().await;
        let mut physical_port_id_changed = proxy.receive_physical_port_id_changed().await;
        let mut mtu_changed = proxy.receive_mtu_changed().await;
        let mut metered_changed = proxy.receive_metered_changed().await;
        let mut real_changed = proxy.receive_real_changed().await;
        let mut ip4_connectivity_changed = proxy.receive_ip4_connectivity_changed().await;
        let mut ip6_connectivity_changed = proxy.receive_ip6_connectivity_changed().await;
        let mut interface_flags_changed = proxy.receive_interface_flags_changed().await;
        let mut hw_address_changed = proxy.receive_hw_address_changed().await;
        let mut ports_changed = proxy.receive_ports_changed().await;

        loop {
            let Some(device) = weak.upgrade() else {
                debug!("Device dropped, stopping monitor");
                return;
            };

            tokio::select! {
                Some(change) = udi_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.udi.set(value);
                    }
                }
                Some(change) = udev_path_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.udev_path.set(value);
                    }
                }
                Some(change) = interface_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.interface.set(value);
                    }
                }
                Some(change) = ip_interface_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.ip_interface.set(value);
                    }
                }
                Some(change) = driver_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.driver.set(value);
                    }
                }
                Some(change) = driver_version_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.driver_version.set(value);
                    }
                }
                Some(change) = firmware_version_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.firmware_version.set(value);
                    }
                }
                Some(change) = capabilities_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.capabilities.set(NMDeviceCapabilities::from_bits_truncate(value));
                    }
                }
                Some(change) = state_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.state.set(NMDeviceState::from_u32(value));
                    }
                }
                Some(change) = state_reason_changed.next() => {
                    if let Ok((state, reason)) = change.get().await {
                        device.state_reason.set((
                            NMDeviceState::from_u32(state),
                            NMDeviceStateReason::from_u32(reason)
                        ));
                    }
                }
                Some(change) = active_connection_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.active_connection.set(value);
                    }
                }
                Some(change) = ip4_config_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.ip4_config.set(value);
                    }
                }
                Some(change) = dhcp4_config_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.dhcp4_config.set(value);
                    }
                }
                Some(change) = ip6_config_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.ip6_config.set(value);
                    }
                }
                Some(change) = dhcp6_config_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.dhcp6_config.set(value);
                    }
                }
                Some(change) = managed_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.managed.set(value);
                    }
                }
                Some(change) = autoconnect_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.autoconnect.set(value);
                    }
                }
                Some(change) = firmware_missing_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.firmware_missing.set(value);
                    }
                }
                Some(change) = nm_plugin_missing_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.nm_plugin_missing.set(value);
                    }
                }
                Some(change) = device_type_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.device_type.set(NMDeviceType::from_u32(value));
                    }
                }
                Some(change) = available_connections_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.available_connections.set(value);
                    }
                }
                Some(change) = physical_port_id_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.physical_port_id.set(value);
                    }
                }
                Some(change) = mtu_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.mtu.set(value);
                    }
                }
                Some(change) = metered_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.metered.set(NMMetered::from_u32(value));
                    }
                }
                Some(change) = real_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.real.set(value);
                    }
                }
                Some(change) = ip4_connectivity_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.ip4_connectivity.set(NMConnectivityState::from_u32(value));
                    }
                }
                Some(change) = ip6_connectivity_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.ip6_connectivity.set(NMConnectivityState::from_u32(value));
                    }
                }
                Some(change) = interface_flags_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.interface_flags.set(NMDeviceInterfaceFlags::from_bits_truncate(value));
                    }
                }
                Some(change) = hw_address_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.hw_address.set(value);
                    }
                }
                Some(change) = ports_changed.next() => {
                    if let Ok(value) = change.get().await {
                        device.ports.set(value);
                    }
                }

                else => {
                    debug!("All property streams ended for device");
                    break;
                }
            }

            drop(device);
        }

        debug!("Property monitoring ended for device");
    }
}
