use tokio::sync::oneshot;

use crate::services::audio::{
    backend::{commands::Command, types::CommandSender},
    error::AudioError,
    types::{device::DeviceKey, stream::StreamKey},
    volume::types::Volume,
};

/// Controller for audio stream operations.
///
/// Provides stateless methods to control audio streams through the backend.
pub(crate) struct AudioStreamController;

impl AudioStreamController {
    pub(crate) async fn set_volume(
        command_tx: &CommandSender,
        stream_key: StreamKey,
        volume: Volume,
    ) -> Result<(), AudioError> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::SetStreamVolume {
                stream_key,
                volume,
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;

        rx.await
            .map_err(|_| AudioError::BackendCommunicationFailed)?
    }

    /// Set the mute state for an audio stream.
    ///
    /// # Errors
    /// Returns error if backend communication fails or stream operation fails.
    pub(crate) async fn set_mute(
        command_tx: &CommandSender,
        stream_key: StreamKey,
        muted: bool,
    ) -> Result<(), AudioError> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::SetStreamMute {
                stream_key,
                muted,
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;

        rx.await
            .map_err(|_| AudioError::BackendCommunicationFailed)?
    }

    /// Move a stream to a different device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or stream operation fails.
    pub(crate) async fn move_to_device(
        command_tx: &CommandSender,
        stream_key: StreamKey,
        device_key: DeviceKey,
    ) -> Result<(), AudioError> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::MoveStream {
                stream_key,
                device_key,
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;

        rx.await
            .map_err(|_| AudioError::BackendCommunicationFailed)?
    }
}
