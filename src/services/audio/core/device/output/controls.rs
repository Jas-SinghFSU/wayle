use tokio::sync::oneshot;

use crate::services::audio::{
    AudioError, DeviceKey, Volume,
    backend::{commands::Command, types::CommandSender},
};

/// Controller for output device operations.
///
/// Provides stateless methods to control output devices through the backend.
pub(crate) struct OutputDeviceController;

impl OutputDeviceController {
    /// Set the volume for an output device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    pub(crate) async fn set_volume(
        command_tx: &CommandSender,
        device_key: DeviceKey,
        volume: Volume,
    ) -> Result<(), AudioError> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::SetVolume {
                device_key,
                volume,
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;

        rx.await
            .map_err(|_| AudioError::BackendCommunicationFailed)?
    }

    /// Set the mute state for an output device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    pub(crate) async fn set_mute(
        command_tx: &CommandSender,
        device_key: DeviceKey,
        muted: bool,
    ) -> Result<(), AudioError> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::SetMute {
                device_key,
                muted,
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;

        rx.await
            .map_err(|_| AudioError::BackendCommunicationFailed)?
    }

    /// Set the active port for an output device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    pub(crate) async fn set_port(
        command_tx: &CommandSender,
        device_key: DeviceKey,
        port: String,
    ) -> Result<(), AudioError> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::SetPort {
                device_key,
                port,
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;

        rx.await
            .map_err(|_| AudioError::BackendCommunicationFailed)?
    }

    /// Set a device as the default output.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    pub(crate) async fn set_as_default(
        command_tx: &CommandSender,
        device_key: DeviceKey,
    ) -> Result<(), AudioError> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::SetDefaultOutput {
                device_key,
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;

        rx.await
            .map_err(|_| AudioError::BackendCommunicationFailed)?
    }
}
