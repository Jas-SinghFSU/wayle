use wayle_bluetooth::types::agent::PairingRequest;

pub(crate) struct PairingCardInit;

#[derive(Debug)]
pub(crate) enum PairingCardMsg {
    SetRequest {
        request: PairingRequest,
        device_name: String,
        device_icon: &'static str,
        device_type_key: &'static str,
    },
    Clear,
    Confirm,
    Reject,
    Cancel,
}
