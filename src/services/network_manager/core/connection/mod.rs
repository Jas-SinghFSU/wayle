mod monitoring;
use std::sync::Arc;

use crate::services::network_manager::NetworkError;
use crate::{unwrap_bool, unwrap_path, unwrap_string, unwrap_u32, unwrap_vec};
use monitoring::ActiveConnectionMonitor;
use tracing::warn;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::services::{
    common::Property,
    network_manager::{ConnectionActiveProxy, NMActivationStateFlags, NMActiveConnectionState},
};

/// Active network connection in NetworkManager.
///
/// Tracks state and configuration of currently active connections,
/// including devices, IP configuration, and connection properties.
/// Properties update reactively as connection state changes.
#[derive(Debug, Clone)]
pub struct ActiveConnection {
    /// The path of the connection object that this ActiveConnection is using.
    pub connection_path: Property<OwnedObjectPath>,

    /// Specific object associated with the active connection. Reflects the
    /// object used during connection activation, and will not change over the
    /// lifetime of the ActiveConnection once set.
    pub specific_object: Property<OwnedObjectPath>,

    /// The ID of the connection, provided for convenience.
    pub id: Property<String>,

    /// The UUID of the connection, provided for convenience.
    pub uuid: Property<String>,

    /// The type of the connection, provided for convenience.
    pub type_: Property<String>,

    /// Array of object paths representing devices which are part of this active
    /// connection.
    pub devices: Property<Vec<OwnedObjectPath>>,

    /// The state of this active connection.
    pub state: Property<NMActiveConnectionState>,

    /// The state flags of this active connection. See NMActivationStateFlags.
    pub state_flags: Property<NMActivationStateFlags>,

    /// Whether this active connection is the default IPv4 connection, i.e. whether it
    /// currently owns the default IPv4 route.
    pub default: Property<bool>,

    /// Object path of the Ip4Config object describing the configuration of the
    /// connection. Only valid when the connection is in the
    /// NM_ACTIVE_CONNECTION_STATE_ACTIVATED state.
    pub ip4_config: Property<OwnedObjectPath>,

    /// Object path of the Dhcp4Config object describing the DHCP options returned by the
    /// DHCP server (assuming the connection used DHCP). Only valid when the connection is
    /// in the NM_ACTIVE_CONNECTION_STATE_ACTIVATED state.
    pub dhcp4_config: Property<OwnedObjectPath>,

    /// Whether this active connection is the default IPv6 connection, i.e. whether it
    /// currently owns the default IPv6 route.
    pub default6: Property<bool>,

    /// Object path of the Ip6Config object describing the configuration of the
    /// connection. Only valid when the connection is in the
    /// NM_ACTIVE_CONNECTION_STATE_ACTIVATED state.
    pub ip6_config: Property<OwnedObjectPath>,

    /// Object path of the Dhcp6Config object describing the DHCP options returned by the
    /// DHCP server (assuming the connection used DHCP). Only valid when the connection is
    /// in the NM_ACTIVE_CONNECTION_STATE_ACTIVATED state.
    pub dhcp6_config: Property<OwnedObjectPath>,

    /// Whether this active connection is also a VPN connection.
    pub vpn: Property<bool>,

    /// The path to the controller device if the connection is a port.
    pub controller: Property<OwnedObjectPath>,
}

impl ActiveConnection {
    /// Get a snapshot of the current connection state (no monitoring).
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::ObjectNotFound` if connection doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    pub async fn get(
        connection: &Connection,
        path: OwnedObjectPath,
    ) -> Result<Arc<Self>, NetworkError> {
        Self::from_path(connection, path).await
    }

    /// Get a live-updating connection instance (with monitoring).
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::ObjectNotFound` if connection doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    pub async fn get_live(
        connection: &Connection,
        path: OwnedObjectPath,
    ) -> Result<Arc<Self>, NetworkError> {
        let active_connection = Self::from_path(connection, path.clone()).await?;

        ActiveConnectionMonitor::start(active_connection.clone(), connection, path).await;

        Ok(active_connection)
    }

    async fn from_path(
        connection: &Connection,
        path: OwnedObjectPath,
    ) -> Result<Arc<Self>, NetworkError> {
        let connection_proxy = ConnectionActiveProxy::new(connection, path.clone()).await?;

        if connection_proxy.connection().await.is_err() {
            warn!(
                "Active Connection at path '{}' does not exist.",
                path.clone()
            );
            return Err(NetworkError::ObjectNotFound(path.to_string()));
        }

        let (
            connection_path,
            specific_object,
            id,
            uuid,
            type_,
            devices,
            state,
            state_flags,
            default,
            ip4_config,
            dhcp4_config,
            default6,
            ip6_config,
            dhcp6_config,
            vpn,
            controller,
        ) = tokio::join!(
            connection_proxy.connection(),
            connection_proxy.specific_object(),
            connection_proxy.id(),
            connection_proxy.uuid(),
            connection_proxy.type_(),
            connection_proxy.devices(),
            connection_proxy.state(),
            connection_proxy.state_flags(),
            connection_proxy.default(),
            connection_proxy.ip4_config(),
            connection_proxy.dhcp4_config(),
            connection_proxy.default6(),
            connection_proxy.ip6_config(),
            connection_proxy.dhcp6_config(),
            connection_proxy.vpn(),
            connection_proxy.controller(),
        );

        let connection_path = unwrap_path!(connection_path, path);
        let specific_object = unwrap_path!(specific_object, path);
        let id = unwrap_string!(id, path);
        let uuid = unwrap_string!(uuid, path);
        let type_ = unwrap_string!(type_, path);
        let devices = unwrap_vec!(devices, path);
        let state = NMActiveConnectionState::from_u32(unwrap_u32!(state, path));
        let state_flags =
            NMActivationStateFlags::from_bits_truncate(unwrap_u32!(state_flags, path));
        let default = unwrap_bool!(default, path);
        let ip4_config = unwrap_path!(ip4_config, path);
        let dhcp4_config = unwrap_path!(dhcp4_config, path);
        let default6 = unwrap_bool!(default6, path);
        let ip6_config = unwrap_path!(ip6_config, path);
        let dhcp6_config = unwrap_path!(dhcp6_config, path);
        let vpn = unwrap_bool!(vpn, path);
        let controller = unwrap_path!(controller, path);

        Ok(Arc::new(Self {
            connection_path: Property::new(connection_path),
            specific_object: Property::new(specific_object),
            id: Property::new(id),
            uuid: Property::new(uuid),
            type_: Property::new(type_),
            devices: Property::new(devices),
            state: Property::new(state),
            state_flags: Property::new(state_flags),
            default: Property::new(default),
            ip4_config: Property::new(ip4_config),
            dhcp4_config: Property::new(dhcp4_config),
            default6: Property::new(default6),
            ip6_config: Property::new(ip6_config),
            dhcp6_config: Property::new(dhcp6_config),
            vpn: Property::new(vpn),
            controller: Property::new(controller),
        }))
    }
}
