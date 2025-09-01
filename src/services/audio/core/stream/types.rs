use tokio_util::sync::CancellationToken;

use crate::services::audio::{
    backend::types::{CommandSender, EventSender},
    types::StreamKey,
};

pub(crate) struct AudioStreamParams<'a> {
    pub command_tx: &'a CommandSender,
    pub stream_key: StreamKey,
}

pub(crate) struct LiveAudioStreamParams<'a> {
    pub command_tx: &'a CommandSender,
    pub event_tx: &'a EventSender,
    pub stream_key: StreamKey,
    pub cancellation_token: CancellationToken,
}
