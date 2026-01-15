use crate::{
    error::{Error, ResponderDropped},
    service::BluetoothService,
    types::agent::{PairingRequest, PairingResponder},
};

pub(crate) async fn pin(service: &BluetoothService, pin: String) -> Result<(), Error> {
    if !matches!(
        service.pairing_request.get(),
        Some(PairingRequest::RequestPinCode { .. })
    ) {
        return Err(Error::NoPendingRequest {
            request_type: "pin",
        });
    }

    let responder = {
        let mut responder_guard = service.pairing_responder.lock().await;
        let Some(pairing_responder) = responder_guard.take() else {
            return Err(Error::NoResponder {
                request_type: "pin",
            });
        };

        let PairingResponder::Pin(responder_channel) = pairing_responder else {
            return Err(Error::NoResponder {
                request_type: "pin",
            });
        };

        responder_channel
    };

    responder.send(pin).map_err(|_| Error::ResponderSend {
        request_type: "pin",
        source: Box::new(ResponderDropped),
    })?;

    service.pairing_request.set(None);

    Ok(())
}

pub(crate) async fn passkey(service: &BluetoothService, passkey: u32) -> Result<(), Error> {
    if !matches!(
        service.pairing_request.get(),
        Some(PairingRequest::RequestPasskey { .. })
    ) {
        return Err(Error::NoPendingRequest {
            request_type: "passkey",
        });
    }

    let responder = {
        let mut responder_guard = service.pairing_responder.lock().await;
        let Some(pairing_responder) = responder_guard.take() else {
            return Err(Error::NoResponder {
                request_type: "passkey",
            });
        };

        let PairingResponder::Passkey(responder_channel) = pairing_responder else {
            return Err(Error::NoResponder {
                request_type: "passkey",
            });
        };

        responder_channel
    };

    responder.send(passkey).map_err(|_| Error::ResponderSend {
        request_type: "passkey",
        source: Box::new(ResponderDropped),
    })?;

    service.pairing_request.set(None);

    Ok(())
}

pub(crate) async fn confirmation(
    service: &BluetoothService,
    confirmation: bool,
) -> Result<(), Error> {
    if !matches!(
        service.pairing_request.get(),
        Some(PairingRequest::RequestConfirmation { .. })
    ) {
        return Err(Error::NoPendingRequest {
            request_type: "confirmation",
        });
    }

    let responder = {
        let mut responder_guard = service.pairing_responder.lock().await;
        let Some(pairing_responder) = responder_guard.take() else {
            return Err(Error::NoResponder {
                request_type: "confirmation",
            });
        };

        let PairingResponder::Confirmation(responder_channel) = pairing_responder else {
            return Err(Error::NoResponder {
                request_type: "confirmation",
            });
        };

        responder_channel
    };

    responder
        .send(confirmation)
        .map_err(|_| Error::ResponderSend {
            request_type: "confirmation",
            source: Box::new(ResponderDropped),
        })?;

    service.pairing_request.set(None);

    Ok(())
}

pub(crate) async fn authorization(
    service: &BluetoothService,
    authorization: bool,
) -> Result<(), Error> {
    if !matches!(
        service.pairing_request.get(),
        Some(PairingRequest::RequestAuthorization { .. })
    ) {
        return Err(Error::NoPendingRequest {
            request_type: "authorization",
        });
    }

    let responder = {
        let mut responder_guard = service.pairing_responder.lock().await;
        let Some(pairing_responder) = responder_guard.take() else {
            return Err(Error::NoResponder {
                request_type: "authorization",
            });
        };

        let PairingResponder::Authorization(responder_channel) = pairing_responder else {
            return Err(Error::NoResponder {
                request_type: "authorization",
            });
        };

        responder_channel
    };

    responder
        .send(authorization)
        .map_err(|_| Error::ResponderSend {
            request_type: "authorization",
            source: Box::new(ResponderDropped),
        })?;

    service.pairing_request.set(None);

    Ok(())
}

pub(crate) async fn service_authorization(
    service: &BluetoothService,
    authorization: bool,
) -> Result<(), Error> {
    if !matches!(
        service.pairing_request.get(),
        Some(PairingRequest::RequestServiceAuthorization { .. })
    ) {
        return Err(Error::NoPendingRequest {
            request_type: "service authorization",
        });
    }

    let responder = {
        let mut responder_guard = service.pairing_responder.lock().await;
        let Some(pairing_responder) = responder_guard.take() else {
            return Err(Error::NoResponder {
                request_type: "service authorization",
            });
        };

        let PairingResponder::ServiceAuthorization(responder_channel) = pairing_responder else {
            return Err(Error::NoResponder {
                request_type: "service authorization",
            });
        };

        responder_channel
    };

    responder
        .send(authorization)
        .map_err(|_| Error::ResponderSend {
            request_type: "service authorization",
            source: Box::new(ResponderDropped),
        })?;

    service.pairing_request.set(None);

    Ok(())
}
