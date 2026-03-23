use wayle_bluetooth::BluetoothService;
use wayle_config::schemas::modules::BluetoothConfig;

use super::{
    BluetoothModule,
    helpers::{BluetoothContext, format_label, select_icon},
};

impl BluetoothModule {
    pub(super) fn compute_display(
        config: &BluetoothConfig,
        bt: &BluetoothService,
    ) -> (String, String) {
        let available = bt.available.get();
        let enabled = bt.enabled.get();
        let devices = bt.devices.get();
        let connected_addresses = bt.connected.get();

        let discovering = bt
            .primary_adapter
            .get()
            .map(|a| a.discovering.get())
            .unwrap_or(false);

        let connected_devices: Vec<_> = devices
            .iter()
            .filter(|d| connected_addresses.contains(&d.address.get()))
            .cloned()
            .collect();

        let ctx = BluetoothContext {
            available,
            enabled,
            discovering,
            connected_devices: &connected_devices,
        };

        (select_icon(config, &ctx), format_label(&ctx))
    }
}
