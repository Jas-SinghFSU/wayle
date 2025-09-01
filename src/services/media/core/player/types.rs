use tokio_util::sync::CancellationToken;
use zbus::Connection;

use crate::services::media::types::PlayerId;

pub(crate) struct PlayerParams<'a> {
    pub connection: &'a Connection,
    pub player_id: PlayerId,
}

pub(crate) struct LivePlayerParams<'a> {
    pub connection: &'a Connection,
    pub player_id: PlayerId,
    pub cancellation_token: CancellationToken,
}
