use std::sync::Arc;

use tokio::sync::{Mutex, mpsc};
use tokio_util::sync::CancellationToken;
use tracing::{debug, info};

use crate::services::{
    bluetooth::{
        error::BluetoothError,
        types::agent::{AgentEvent, PairingRequest, PairingResponder},
    },
    common::property::Property,
};

pub(crate) async fn start(
    mut agent_tx: mpsc::UnboundedReceiver<AgentEvent>,
    pairing_responder: &Arc<Mutex<Option<PairingResponder>>>,
    pairing_request: &Property<Option<PairingRequest>>,
    cancellation_token: CancellationToken,
) -> Result<(), BluetoothError> {
    let pairing_responder = pairing_responder.clone();
    let pairing_request = pairing_request.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Agent event processor cancelled");
                    return;
                }
                Some(agent_event) = agent_tx.recv() => {
                    match agent_event {
                        AgentEvent::PinRequested { device_path, responder } => {
                            *pairing_responder
                                .lock().await = Some(PairingResponder::Pin(responder));
                            pairing_request
                                .set(Some(PairingRequest::RequestPinCode { device_path }));
                        },
                        AgentEvent::PasskeyRequested { device_path, responder } => {
                            *pairing_responder
                                .lock().await = Some(PairingResponder::Passkey(responder));
                            pairing_request
                                .set(Some(PairingRequest::RequestPasskey { device_path }));
                        },
                        AgentEvent::ConfirmationRequested { device_path, passkey, responder } => {
                            *pairing_responder
                                .lock().await = Some(PairingResponder::Confirmation(responder));
                            pairing_request.set(Some(PairingRequest::RequestConfirmation {
                                device_path,
                                passkey,
                            }));
                        },
                        AgentEvent::AuthorizationRequested { device_path, responder } => {
                            *pairing_responder
                                .lock().await = Some(PairingResponder::Authorization(responder));
                            pairing_request.set(Some(PairingRequest::RequestAuthorization {
                                device_path
                            }));
                        },
                        AgentEvent::ServiceAuthorizationRequested {
                            device_path,
                            uuid,
                            responder
                        } => {
                            *pairing_responder
                                .lock().await = Some(PairingResponder::ServiceAuthorization(responder));
                            pairing_request.set(Some(PairingRequest::RequestServiceAuthorization {
                                device_path,
                                uuid
                            }));
                        },
                        AgentEvent::DisplayPinCode { device_path, pincode } => {
                            pairing_request.set(Some(PairingRequest::DisplayPinCode {
                                device_path,
                                pincode
                            }));
                        },
                        AgentEvent::DisplayPasskey { device_path, passkey, entered } => {
                            pairing_request.set(Some(PairingRequest::DisplayPasskey {
                                device_path,
                                passkey,
                                entered
                            }));
                        },
                        AgentEvent::Cancelled => {
                            info!("Pairing cancelled");
                            *pairing_responder.lock().await = None;
                            pairing_request.set(None);
                        },
                    }
                }
            }
        }
    });

    Ok(())
}
