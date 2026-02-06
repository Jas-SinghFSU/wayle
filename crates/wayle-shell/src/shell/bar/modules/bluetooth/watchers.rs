use std::sync::Arc;

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_bluetooth::BluetoothService;
use wayle_common::{watch, watch_cancellable};
use wayle_config::schemas::modules::BluetoothConfig;

use super::{BluetoothModule, messages::BluetoothCmd};

pub(super) fn spawn_watchers(
    sender: &ComponentSender<BluetoothModule>,
    config: &BluetoothConfig,
    bt: &Arc<BluetoothService>,
) {
    let available = bt.available.clone();
    let enabled = bt.enabled.clone();
    let connected = bt.connected.clone();
    let devices = bt.devices.clone();

    watch!(
        sender,
        [
            available.watch(),
            enabled.watch(),
            connected.watch(),
            devices.watch()
        ],
        |out| {
            let _ = out.send(BluetoothCmd::StateChanged);
        }
    );

    let primary_adapter = bt.primary_adapter.clone();
    watch!(sender, [primary_adapter.watch()], |out| {
        let _ = out.send(BluetoothCmd::AdapterChanged);
    });

    let disabled_icon = config.disabled_icon.clone();
    let disconnected_icon = config.disconnected_icon.clone();
    let connected_icon = config.connected_icon.clone();
    let searching_icon = config.searching_icon.clone();

    watch!(
        sender,
        [
            disabled_icon.watch(),
            disconnected_icon.watch(),
            connected_icon.watch(),
            searching_icon.watch()
        ],
        |out| {
            let _ = out.send(BluetoothCmd::IconConfigChanged);
        }
    );
}

pub(super) fn spawn_adapter_watchers(
    sender: &ComponentSender<BluetoothModule>,
    token: CancellationToken,
    bt: &Arc<BluetoothService>,
) {
    if let Some(adapter) = bt.primary_adapter.get() {
        let discovering = adapter.discovering.clone();
        watch_cancellable!(sender, token, [discovering.watch()], |out| {
            let _ = out.send(BluetoothCmd::StateChanged);
        });
    }
}
