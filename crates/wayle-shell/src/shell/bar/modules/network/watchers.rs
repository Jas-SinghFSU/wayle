use relm4::ComponentSender;
use wayle_common::{services, watch};
use wayle_config::schemas::modules::NetworkConfig;
use wayle_network::NetworkService;

use super::{NetworkModule, messages::NetworkCmd};

pub(super) fn spawn_watchers(sender: &ComponentSender<NetworkModule>, config: &NetworkConfig) {
    let network_service = services::get::<NetworkService>();

    let primary = network_service.primary.clone();
    watch!(sender, [primary.watch()], |out| {
        let _ = out.send(NetworkCmd::StateChanged);
    });

    if let Some(wifi) = &network_service.wifi {
        let enabled = wifi.enabled.clone();
        let connectivity = wifi.connectivity.clone();
        let ssid = wifi.ssid.clone();
        let strength = wifi.strength.clone();
        watch!(
            sender,
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

    if let Some(wired) = &network_service.wired {
        let connectivity = wired.connectivity.clone();
        watch!(sender, [connectivity.watch()], |out| {
            let _ = out.send(NetworkCmd::StateChanged);
        });
    }

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
