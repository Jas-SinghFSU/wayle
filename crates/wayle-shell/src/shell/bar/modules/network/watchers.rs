use std::sync::Arc;

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_common::{watch, watch_cancellable};
use wayle_config::schemas::modules::NetworkConfig;
use wayle_network::NetworkService;

use super::{NetworkModule, messages::NetworkCmd};

pub(super) fn spawn_watchers(
    sender: &ComponentSender<NetworkModule>,
    config: &NetworkConfig,
    network: &Arc<NetworkService>,
) {
    let primary = network.primary.clone();
    watch!(sender, [primary.watch()], |out| {
        let _ = out.send(NetworkCmd::StateChanged);
    });

    let wifi = network.wifi.clone();
    watch!(sender, [wifi.watch()], |out| {
        let _ = out.send(NetworkCmd::WifiDeviceChanged);
    });

    let wired = network.wired.clone();
    watch!(sender, [wired.watch()], |out| {
        let _ = out.send(NetworkCmd::WiredDeviceChanged);
    });

    spawn_icon_config_watchers(sender, config);
}

pub(super) fn spawn_wifi_watchers(
    sender: &ComponentSender<NetworkModule>,
    network: &Arc<NetworkService>,
    token: CancellationToken,
) {
    let Some(wifi) = network.wifi.get() else {
        return;
    };

    let enabled = wifi.enabled.clone();
    let connectivity = wifi.connectivity.clone();
    let ssid = wifi.ssid.clone();
    let strength = wifi.strength.clone();

    watch_cancellable!(
        sender,
        token,
        [
            enabled.watch(),
            connectivity.watch(),
            ssid.watch(),
            strength.watch()
        ],
        |out| {
            let _ = out.send(NetworkCmd::StateChanged);
        }
    );
}

pub(super) fn spawn_wired_watchers(
    sender: &ComponentSender<NetworkModule>,
    network: &Arc<NetworkService>,
    token: CancellationToken,
) {
    let Some(wired) = network.wired.get() else {
        return;
    };

    let connectivity = wired.connectivity.clone();

    watch_cancellable!(sender, token, [connectivity.watch()], |out| {
        let _ = out.send(NetworkCmd::StateChanged);
    });
}

fn spawn_icon_config_watchers(sender: &ComponentSender<NetworkModule>, config: &NetworkConfig) {
    let wifi_disabled_icon = config.wifi_disabled_icon.clone();
    let wifi_acquiring_icon = config.wifi_acquiring_icon.clone();
    let wifi_offline_icon = config.wifi_offline_icon.clone();
    let wifi_connected_icon = config.wifi_connected_icon.clone();
    let wifi_signal_icons = config.wifi_signal_icons.clone();
    let wired_connected_icon = config.wired_connected_icon.clone();
    let wired_acquiring_icon = config.wired_acquiring_icon.clone();
    let wired_disconnected_icon = config.wired_disconnected_icon.clone();

    watch!(
        sender,
        [
            wifi_disabled_icon.watch(),
            wifi_acquiring_icon.watch(),
            wifi_offline_icon.watch(),
            wifi_connected_icon.watch(),
            wifi_signal_icons.watch(),
            wired_connected_icon.watch(),
            wired_acquiring_icon.watch(),
            wired_disconnected_icon.watch()
        ],
        |out| {
            let _ = out.send(NetworkCmd::IconConfigChanged);
        }
    );
}
