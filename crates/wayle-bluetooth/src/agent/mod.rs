pub(crate) mod event_processor;
pub(crate) mod providers;
use tokio::sync::{mpsc::UnboundedSender, oneshot};
use zbus::{fdo, interface, zvariant::OwnedObjectPath};

use super::types::agent::AgentEvent;

pub(crate) struct BluetoothAgent {
    pub service_tx: UnboundedSender<AgentEvent>,
}

#[interface(name = "org.bluez.Agent1")]
impl BluetoothAgent {
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

    async fn display_pincode(&self, device: OwnedObjectPath, pincode: String) -> fdo::Result<()> {
        self.service_tx
            .send(AgentEvent::DisplayPinCode {
                device_path: device,
                pincode,
            })
            .map_err(|e| fdo::Error::Failed(format!("Service unavailable: {e}")))
    }

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

    async fn cancel(&self) -> fdo::Result<()> {
        self.service_tx
            .send(AgentEvent::Cancelled)
            .map_err(|e| fdo::Error::Failed(format!("Service unavailable: {e}")))?;

        Ok(())
    }
}
