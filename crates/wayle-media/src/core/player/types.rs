use tokio_util::sync::CancellationToken;
use zbus::Connection;

use crate::types::PlayerId;

/// Parameters for creating a Player instance.
///
/// **Note**: This type is exposed for trait implementation requirements
/// but should not be constructed directly by external consumers.
#[doc(hidden)]
pub struct PlayerParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) player_id: PlayerId,
}

/// Parameters for creating a live Player instance.
///
/// **Note**: This type is exposed for trait implementation requirements
/// but should not be constructed directly by external consumers.
#[doc(hidden)]
pub struct LivePlayerParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) player_id: PlayerId,
    pub(crate) cancellation_token: &'a CancellationToken,
}
