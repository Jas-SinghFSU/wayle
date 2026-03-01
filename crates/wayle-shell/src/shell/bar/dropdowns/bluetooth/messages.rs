use std::sync::Arc;

use wayle_bluetooth::{BluetoothService, types::agent::PairingRequest};
use wayle_config::ConfigService;
use zbus::zvariant::OwnedObjectPath;

pub(crate) struct BluetoothDropdownInit {
    pub bluetooth: Arc<BluetoothService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum BluetoothDropdownMsg {
    BluetoothToggled(bool),
    ScanRequested,
    DeviceAction(DeviceActionMsg),
    PairingCard(PairingCardOutput),
}

#[derive(Debug)]
pub(crate) enum DeviceActionMsg {
    Connect(OwnedObjectPath),
    Disconnect(OwnedObjectPath),
    Forget(OwnedObjectPath),
}

#[derive(Debug)]
pub(crate) enum PairingCardOutput {
    Cancelled,
    PinSubmitted(String),
    PasskeyConfirmed,
    PasskeyRejected,
    AuthorizationAccepted,
    AuthorizationRejected,
    ServiceAuthorizationAccepted,
    ServiceAuthorizationRejected,
    LegacyPinSubmitted(String),
}

#[derive(Debug)]
pub(crate) enum BluetoothDropdownCmd {
    ScaleChanged(f32),
    EnabledChanged(bool),
    AvailableChanged(bool),
    ScanComplete,
    DevicesChanged,
    DevicePropertyChanged,
    DeviceActionFailed(OwnedObjectPath),
    PairingRequested(Option<PairingRequest>),
}
