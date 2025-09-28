use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::services::network::types::states::{
    NMActiveConnectionState, NMActiveConnectionStateReason, NMVpnConnectionState,
    NMVpnConnectionStateReason,
};

pub(crate) struct ActiveConnectionParams<'a> {
    pub connection: &'a Connection,
    pub path: OwnedObjectPath,
}

pub(crate) struct LiveActiveConnectionParams<'a> {
    pub connection: &'a Connection,
    pub path: OwnedObjectPath,
    pub cancellation_token: &'a CancellationToken,
}

/// Event emitted when the active connection changes state.
pub struct ActiveConnectionStateChangedEvent {
    /// The new connection state.
    pub state: NMActiveConnectionState,
    /// The reason for the state change.
    pub reason: NMActiveConnectionStateReason,
}

/// Event emitted when the state of the VPN connection has changed.
pub struct VpnConnectionStateChangedEvent {
    /// The new VPN connection state.
    pub state: NMVpnConnectionState,
    /// The reason for the state change.
    pub reason: NMVpnConnectionStateReason,
}
