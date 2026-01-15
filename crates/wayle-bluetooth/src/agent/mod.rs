pub(crate) mod event_processor;
pub(crate) mod providers;
use tokio::sync::{mpsc::UnboundedSender, oneshot};
use zbus::{fdo, interface, zvariant::OwnedObjectPath};

use super::types::agent::AgentEvent;

/// D-Bus Agent1 interface implementation for handling Bluetooth pairing and authentication.
///
/// Receives pairing requests from BlueZ and forwards them to the Bluetooth service
/// for routing to the appropriate device. Blocks waiting for user response via
/// oneshot channels.
///
/// # Errors
///
/// Agent methods return D-Bus errors when pairing operations fail or are rejected.
pub(crate) struct BluetoothAgent {
    pub service_tx: UnboundedSender<AgentEvent>,
}

#[interface(name = "org.bluez.Agent1")]
impl BluetoothAgent {
    /// Called when bluetoothd needs to get the passkey for an authentication.
    ///
    /// Returns a string of 1-16 alphanumeric characters.
    ///
    /// # Errors
    ///
    /// - `Rejected` - Request rejected by user
    /// - `Canceled` - Request canceled
    async fn request_pin_code(&self, device: OwnedObjectPath) -> fdo::Result<String> {
        let (response_tx, response_rx) = oneshot::channel();

        self.service_tx
            .send(AgentEvent::PinRequested {
                device_path: device,
                responder: response_tx,
            })
            .map_err(|e| fdo::Error::Failed(format!("Service unavailable: {e}")))?;

        response_rx
            .await
            .map_err(|e| fdo::Error::Failed(format!("User cancelled: {e}")))
    }

    /// Called when bluetoothd needs to display a pincode for an authentication.
    ///
    /// Returns an empty reply. When the pincode no longer needs to be displayed, the
    /// Cancel method of the agent is called.
    ///
    /// This is used during the pairing process of keyboards that don't support
    /// Bluetooth 2.1 Secure Simple Pairing, in contrast to DisplayPasskey which is used
    /// for those that do.
    ///
    /// This method will only ever be called once since older keyboards do not support
    /// typing notification.
    ///
    /// Note that the PIN will always be a 6-digit number, zero-padded to 6 digits. This
    /// is for harmony with the later specification.
    ///
    /// # Errors
    ///
    /// - `Rejected` - Request rejected
    /// - `Canceled` - Request canceled
    async fn display_pincode(&self, device: OwnedObjectPath, pincode: String) -> fdo::Result<()> {
        self.service_tx
            .send(AgentEvent::DisplayPinCode {
                device_path: device,
                pincode,
            })
            .map_err(|e| fdo::Error::Failed(format!("Service unavailable: {e}")))
    }

    /// Called when bluetoothd needs to get the passkey for an authentication.
    ///
    /// Returns a numeric value between 0-999999.
    ///
    /// # Errors
    ///
    /// - `Rejected` - Request rejected by user
    /// - `Canceled` - Request canceled
    async fn request_passkey(&self, device: OwnedObjectPath) -> fdo::Result<u32> {
        let (response_tx, response_rx) = oneshot::channel();

        self.service_tx
            .send(AgentEvent::PasskeyRequested {
                device_path: device,
                responder: response_tx,
            })
            .map_err(|e| fdo::Error::Failed(format!("Service unavailable: {e}")))?;

        response_rx
            .await
            .map_err(|e| fdo::Error::Failed(format!("User cancelled: {e}")))
    }

    /// Called when bluetoothd needs to display a passkey for an authentication.
    ///
    /// The entered parameter indicates the number of already typed keys on the remote
    /// side.
    ///
    /// Returns an empty reply. When the passkey no longer needs to be displayed, the
    /// Cancel method of the agent is called.
    ///
    /// During the pairing process this method might be called multiple times to update
    /// the entered value.
    ///
    /// Note that the passkey will always be a 6-digit number, so the display should be
    /// zero-padded at the start if the value contains less than 6 digits.
    async fn display_passkey(
        &self,
        device: OwnedObjectPath,
        passkey: u32,
        entered: u16,
    ) -> fdo::Result<()> {
        self.service_tx
            .send(AgentEvent::DisplayPasskey {
                device_path: device,
                passkey,
                entered,
            })
            .map_err(|e| fdo::Error::Failed(format!("Service unavailable: {e}")))
    }

    /// Called when bluetoothd needs to confirm a passkey for an authentication.
    ///
    /// Returns an empty reply if the passkey is confirmed, or an error if rejected.
    ///
    /// Note that the passkey will always be a 6-digit number, so the display should be
    /// zero-padded at the start if the value contains less than 6 digits.
    ///
    /// # Errors
    ///
    /// - `Rejected` - Passkey rejected
    /// - `Canceled` - Request canceled
    async fn request_confirmation(&self, device: OwnedObjectPath, passkey: u32) -> fdo::Result<()> {
        let (response_tx, response_rx) = oneshot::channel();

        self.service_tx
            .send(AgentEvent::ConfirmationRequested {
                device_path: device,
                passkey,
                responder: response_tx,
            })
            .map_err(|e| fdo::Error::Failed(format!("Service unavailable: {e}")))?;

        match response_rx.await {
            Ok(true) => Ok(()),
            _ => Err(fdo::Error::Failed("User rejected".into())),
        }
    }

    /// Called to request user authorization for an incoming pairing attempt that would
    /// otherwise trigger the just-works model, or when a device implementing cable
    /// pairing is plugged in. In the latter case, the device may not yet be connected
    /// to the adapter via Bluetooth.
    ///
    /// # Errors
    ///
    /// - `Rejected` - Authorization rejected
    /// - `Canceled` - Request canceled
    async fn request_authorization(&self, device: OwnedObjectPath) -> fdo::Result<()> {
        let (response_tx, response_rx) = oneshot::channel();

        self.service_tx
            .send(AgentEvent::AuthorizationRequested {
                device_path: device,
                responder: response_tx,
            })
            .map_err(|e| fdo::Error::Failed(format!("Service unavailable: {e}")))?;

        match response_rx.await {
            Ok(true) => Ok(()),
            _ => Err(fdo::Error::Failed("User rejected".into())),
        }
    }

    /// Called when bluetoothd needs to authorize a connection/service request.
    ///
    /// # Errors
    ///
    /// - `Rejected` - Service authorization rejected
    /// - `Canceled` - Request canceled
    async fn authorize_service(&self, device: OwnedObjectPath, uuid: String) -> fdo::Result<()> {
        let (response_tx, response_rx) = oneshot::channel();

        self.service_tx
            .send(AgentEvent::ServiceAuthorizationRequested {
                device_path: device,
                uuid,
                responder: response_tx,
            })
            .map_err(|e| fdo::Error::Failed(format!("Service unavailable: {e}")))?;

        match response_rx.await {
            Ok(true) => Ok(()),
            _ => Err(fdo::Error::Failed("User rejected".into())),
        }
    }

    /// Called to indicate that the agent request failed before a reply was returned.
    async fn cancel(&self) -> fdo::Result<()> {
        self.service_tx
            .send(AgentEvent::Cancelled)
            .map_err(|e| fdo::Error::Failed(format!("Service unavailable: {e}")))?;

        Ok(())
    }
}
