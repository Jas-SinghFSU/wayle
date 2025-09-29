use crate::{
    error::Error,
    service::BluetoothService,
    types::agent::{PairingRequest, PairingResponder},
};

pub(crate) async fn pin(service: &BluetoothService, pin: String) -> Result<(), Error> {
    if !matches!(
        service.pairing_request.get(),
        Some(PairingRequest::RequestPinCode { .. })
    ) {
        return Err(Error::OperationFailed {
            operation: "pin",
            reason: String::from("Failed to provide PIN. A PIN is not currently being requested."),
        });
    }

    let responder = {
        let mut responder_guard = service.pairing_responder.lock().await;
        let Some(pairing_responder) = responder_guard.take() else {
            return Err(Error::OperationFailed {
                operation: "pin",
                reason: String::from("Failed to provide PIN. No pairing responder available."),
            });
        };

        let PairingResponder::Pin(responder_channel) = pairing_responder else {
            return Err(Error::OperationFailed {
                operation: "pin",
                reason: String::from("Failed to provide PIN. No PIN responder available."),
            });
        };

        responder_channel
    };

    responder.send(pin).map_err(|err| Error::OperationFailed {
        operation: "pin",
        reason: format!("Failed to process PIN: {err}"),
    })?;

    service.pairing_request.set(None);

    Ok(())
}

pub(crate) async fn passkey(service: &BluetoothService, passkey: u32) -> Result<(), Error> {
    if !matches!(
        service.pairing_request.get(),
        Some(PairingRequest::RequestPasskey { .. })
    ) {
        return Err(Error::OperationFailed {
            operation: "passkey",
            reason: String::from(
                "Failed to provide passkey. A passkey is not currently being requested.",
            ),
        });
    }

    let responder = {
        let mut responder_guard = service.pairing_responder.lock().await;
        let Some(pairing_responder) = responder_guard.take() else {
            return Err(Error::OperationFailed {
                operation: "passkey",
                reason: String::from("Failed to provide passkey. No pairing responder available."),
            });
        };

        let PairingResponder::Passkey(responder_channel) = pairing_responder else {
            return Err(Error::OperationFailed {
                operation: "passkey",
                reason: String::from("Failed to provide passkey. No passkey responder available."),
            });
        };

        responder_channel
    };

    responder
        .send(passkey)
        .map_err(|err| Error::OperationFailed {
            operation: "passkey",
            reason: format!("Failed to process passkey: {err}"),
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
        return Err(Error::OperationFailed {
            operation: "confirmation",
            reason: String::from(
                "Failed to provide confirmation. A confirmation is not currently being requested.",
            ),
        });
    }

    let responder = {
        let mut responder_guard = service.pairing_responder.lock().await;
        let Some(pairing_responder) = responder_guard.take() else {
            return Err(Error::OperationFailed {
                operation: "confirmation",
                reason: String::from(
                    "Failed to provide confirmation. No confirmation responder available.",
                ),
            });
        };

        let PairingResponder::Confirmation(responder_channel) = pairing_responder else {
            return Err(Error::OperationFailed {
                operation: "confirmation",
                reason: String::from(
                    "Failed to provide confirmation. No confirmation responder available.",
                ),
            });
        };

        responder_channel
    };

    responder
        .send(confirmation)
        .map_err(|err| Error::OperationFailed {
            operation: "confirmation",
            reason: format!("Failed to process confirmation: {err}"),
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
        return Err(Error::OperationFailed {
            operation: "authorization",
            reason: String::from(
                "Failed to provide authorization. An authorization is not currently being requested.",
            ),
        });
    }

    let responder = {
        let mut responder_guard = service.pairing_responder.lock().await;
        let Some(pairing_responder) = responder_guard.take() else {
            return Err(Error::OperationFailed {
                operation: "authorization",
                reason: String::from(
                    "Failed to provide authorization. No authorization responder available.",
                ),
            });
        };

        let PairingResponder::Authorization(responder_channel) = pairing_responder else {
            return Err(Error::OperationFailed {
                operation: "authorization",
                reason: String::from(
                    "Failed to provide authorization. No authorization responder available.",
                ),
            });
        };

        responder_channel
    };

    responder
        .send(authorization)
        .map_err(|err| Error::OperationFailed {
            operation: "authorization",
            reason: format!("Failed to process authorization: {err}"),
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
        return Err(Error::OperationFailed {
            operation: "service_authorization",
            reason: String::from(
                "Failed to provide service authorization. A service authorization is not currently being requested.",
            ),
        });
    }

    let responder = {
        let mut responder_guard = service.pairing_responder.lock().await;
        let Some(pairing_responder) = responder_guard.take() else {
            return Err(Error::OperationFailed {
                operation: "service_authorization",
                reason: String::from(
                    "Failed to provide service authorization. No service authorization responder available.",
                ),
            });
        };

        let PairingResponder::ServiceAuthorization(responder_channel) = pairing_responder else {
            return Err(Error::OperationFailed {
                operation: "service_authorization",
                reason: String::from(
                    "Failed to provide service authorization. No service authorization responder available.",
                ),
            });
        };

        responder_channel
    };

    responder
        .send(authorization)
        .map_err(|err| Error::OperationFailed {
            operation: "service_authorization",
            reason: format!("Failed to process service authorization: {err}"),
        })?;

    service.pairing_request.set(None);

    Ok(())
}
