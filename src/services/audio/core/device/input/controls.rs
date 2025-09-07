use tokio::sync::oneshot;

use crate::services::audio::{
    backend::{commands::Command, types::CommandSender},
    error::AudioError,
    types::device::DeviceKey,
    volume::types::Volume,
};

/// Controller for input device operations.
///
/// Provides stateless methods to control input devices through the backend.
pub(crate) struct InputDeviceController;

impl InputDeviceController {
    /// Set the volume for an input device.
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

    /// Set the mute state for an input device.
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

    /// Set the active port for an input device.
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

    /// Set a device as the default input.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    pub(crate) async fn set_as_default(
        command_tx: &CommandSender,
        device_key: DeviceKey,
    ) -> Result<(), AudioError> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::SetDefaultInput {
                device_key,
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;

        rx.await
            .map_err(|_| AudioError::BackendCommunicationFailed)?
    }
}
