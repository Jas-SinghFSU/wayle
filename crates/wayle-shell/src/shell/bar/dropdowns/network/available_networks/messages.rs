use std::{collections::HashSet, sync::Arc};

use wayle_network::NetworkService;
use zbus::zvariant::OwnedObjectPath;

use crate::shell::bar::dropdowns::network::password_form::PasswordFormOutput;

pub(super) struct SelectedNetwork {
    pub ap_path: OwnedObjectPath,
    pub ssid: String,
    pub security_label: String,
    pub signal_icon: &'static str,
}

pub(crate) struct AvailableNetworksInit {
    pub network: Arc<NetworkService>,
}

#[derive(Debug)]
pub(crate) enum AvailableNetworksInput {
    ScanRequested,
    WifiAvailabilityChanged(bool),
    WifiEnabledChanged(bool),
    NetworkSelected(usize),
    PasswordForm(PasswordFormOutput),
}

#[derive(Debug)]
pub(crate) enum AvailableNetworksCmd {
    AccessPointsChanged,
    KnownSsidsUpdated(HashSet<String>),
    ConnectionProgress(String),
    ConnectImmediateError(String),
    ConnectionActivated,
    ConnectionAuthFailed,
    ConnectionTimedOut,
    ConnectionFailed(String),
    ScanComplete,
}

#[derive(Debug)]
pub(crate) enum AvailableNetworksOutput {
    ScanStarted,
    ScanComplete,
    Connecting(String),
    ConnectionProgress(String),
    ClearConnecting,
    Connected,
    ConnectionFailed(String),
}
