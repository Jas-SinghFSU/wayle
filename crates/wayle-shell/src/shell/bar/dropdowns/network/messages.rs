use std::sync::Arc;

use wayle_config::ConfigService;
use wayle_network::NetworkService;

use super::available_networks::AvailableNetworksOutput;

pub(crate) struct NetworkDropdownInit {
    pub network: Arc<NetworkService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum NetworkDropdownMsg {
    WifiToggled(bool),
    ScanRequested,
    AvailableNetworks(AvailableNetworksOutput),
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum NetworkDropdownCmd {
    ScaleChanged(f32),
    WifiDeviceChanged,
    WifiEnabledChanged(bool),
}
