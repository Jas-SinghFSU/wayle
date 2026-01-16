use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::types::states::{
    NMActiveConnectionState, NMActiveConnectionStateReason, NMVpnConnectionState,
    NMVpnConnectionStateReason,
};

#[doc(hidden)]
pub struct ActiveConnectionParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) path: OwnedObjectPath,
}

#[doc(hidden)]
pub struct LiveActiveConnectionParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) path: OwnedObjectPath,
    pub(crate) cancellation_token: &'a CancellationToken,
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
