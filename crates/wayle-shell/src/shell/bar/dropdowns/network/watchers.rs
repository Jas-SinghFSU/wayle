use std::sync::Arc;

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_common::{watch, watch_cancellable};
use wayle_config::ConfigService;
use wayle_network::NetworkService;

use super::{NetworkDropdown, messages::NetworkDropdownCmd};

pub(super) fn spawn(
    sender: &ComponentSender<NetworkDropdown>,
    config: &Arc<ConfigService>,
    network: &Arc<NetworkService>,
) {
    let scale = config.config().styling.scale.clone();
    watch!(sender, [scale.watch()], |out| {
        let _ = out.send(NetworkDropdownCmd::ScaleChanged(scale.get().value()));
    });

    let wifi = network.wifi.clone();
    watch!(sender, [wifi.watch()], |out| {
        let _ = out.send(NetworkDropdownCmd::WifiDeviceChanged);
    });
}

pub(super) fn spawn_wifi_watchers(
    sender: &ComponentSender<NetworkDropdown>,
    network: &Arc<NetworkService>,
    token: CancellationToken,
) {
    let Some(wifi) = network.wifi.get() else {
        return;
    };

    let enabled = wifi.enabled.clone();
    watch_cancellable!(sender, token, [enabled.watch()], |out| {
        let _ = out.send(NetworkDropdownCmd::WifiEnabledChanged(enabled.get()));
    });
}
