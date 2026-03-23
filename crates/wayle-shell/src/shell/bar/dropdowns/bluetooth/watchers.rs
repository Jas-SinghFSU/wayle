use std::sync::Arc;

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_bluetooth::BluetoothService;
use wayle_config::ConfigService;
use wayle_widgets::{watch, watch_cancellable};

use super::{BluetoothDropdown, messages::BluetoothDropdownCmd};

pub(super) fn spawn(
    sender: &ComponentSender<BluetoothDropdown>,
    config: &Arc<ConfigService>,
    bluetooth: &Arc<BluetoothService>,
) {
    let scale = config.config().styling.scale.clone();
    watch!(sender, [scale.watch()], |out| {
        let _ = out.send(BluetoothDropdownCmd::ScaleChanged(scale.get().value()));
    });

    let available = bluetooth.available.clone();
    watch!(sender, [available.watch()], |out| {
        let _ = out.send(BluetoothDropdownCmd::AvailableChanged(available.get()));
    });

    let enabled = bluetooth.enabled.clone();
    watch!(sender, [enabled.watch()], |out| {
        let _ = out.send(BluetoothDropdownCmd::EnabledChanged(enabled.get()));
    });

    let devices = bluetooth.devices.clone();
    watch!(sender, [devices.watch()], |out| {
        let _ = out.send(BluetoothDropdownCmd::DevicesChanged);
    });

    let pairing = bluetooth.pairing_request.clone();
    watch!(sender, [pairing.watch()], |out| {
        let _ = out.send(BluetoothDropdownCmd::PairingRequested(pairing.get()));
    });
}

pub(super) fn spawn_device_watchers(
    sender: &ComponentSender<BluetoothDropdown>,
    bluetooth: &Arc<BluetoothService>,
    token: CancellationToken,
) {
    let devices = bluetooth.devices.get();
    for device in &devices {
        let connected = device.connected.clone();
        let paired = device.paired.clone();
        let name = device.name.clone();
        let alias = device.alias.clone();
        let battery = device.battery_percentage.clone();

        watch_cancellable!(
            sender,
            token.clone(),
            [
                connected.watch(),
                paired.watch(),
                name.watch(),
                alias.watch(),
                battery.watch()
            ],
            |out| {
                let _ = out.send(BluetoothDropdownCmd::DevicePropertyChanged);
            }
        );
    }
}
