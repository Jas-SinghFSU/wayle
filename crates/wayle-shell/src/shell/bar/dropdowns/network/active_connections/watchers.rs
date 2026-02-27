use std::sync::Arc;

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_common::{watch, watch_cancellable};
use wayle_network::NetworkService;

use crate::shell::bar::dropdowns::network::active_connections::{
    ActiveConnections, messages::ActiveConnectionsCmd,
};

pub(super) fn spawn_wifi_watchers(
    sender: &ComponentSender<ActiveConnections>,
    network: &Arc<NetworkService>,
    token: CancellationToken,
) {
    let Some(wifi) = network.wifi.get() else {
        return;
    };

    let connectivity = wifi.connectivity.clone();
    let ssid = wifi.ssid.clone();
    let strength = wifi.strength.clone();
    let frequency = wifi.frequency.clone();
    let ip4_address = wifi.ip4_address.clone();

    watch_cancellable!(
        sender,
        token,
        [
            connectivity.watch(),
            ssid.watch(),
            strength.watch(),
            frequency.watch(),
            ip4_address.watch()
        ],
        |out| {
            let _ = out.send(ActiveConnectionsCmd::WifiStateChanged {
                connectivity: connectivity.get(),
                ssid: ssid.get(),
                strength: strength.get(),
                frequency: frequency.get(),
                ip4_address: ip4_address.get(),
            });
        }
    );
}

pub(super) fn spawn_device_watchers(
    sender: &ComponentSender<ActiveConnections>,
    network: &Arc<NetworkService>,
) {
    let wifi = network.wifi.clone();
    watch!(sender, [wifi.watch()], |out| {
        let _ = out.send(ActiveConnectionsCmd::WifiDeviceChanged);
    });

    let wired = network.wired.clone();
    watch!(sender, [wired.watch()], |out| {
        let _ = out.send(ActiveConnectionsCmd::WiredDeviceChanged);
    });
}

pub(super) fn spawn_wired_watchers(
    sender: &ComponentSender<ActiveConnections>,
    network: &Arc<NetworkService>,
    token: CancellationToken,
) {
    let Some(wired) = network.wired.get() else {
        return;
    };

    let connectivity = wired.connectivity.clone();
    let speed = wired.device.speed.clone();
    let ip4_address = wired.ip4_address.clone();

    watch_cancellable!(
        sender,
        token,
        [connectivity.watch(), speed.watch(), ip4_address.watch()],
        |out| {
            let _ = out.send(ActiveConnectionsCmd::WiredStateChanged {
                connectivity: connectivity.get(),
                speed: speed.get(),
                ip4_address: ip4_address.get(),
            });
        }
    );
}
