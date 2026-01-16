use tokio_util::sync::CancellationToken;
use zbus::Connection;

use crate::types::PlayerId;

#[doc(hidden)]
pub struct PlayerParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) player_id: PlayerId,
}

#[doc(hidden)]
pub struct LivePlayerParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) player_id: PlayerId,
    pub(crate) cancellation_token: &'a CancellationToken,
}
