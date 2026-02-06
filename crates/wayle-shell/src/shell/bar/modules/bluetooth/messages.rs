use std::sync::Arc;

use wayle_bluetooth::BluetoothService;
use wayle_config::ConfigService;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct BluetoothInit {
    pub settings: BarSettings,
    pub bluetooth: Arc<BluetoothService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum BluetoothMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum BluetoothCmd {
    StateChanged,
    IconConfigChanged,
    AdapterChanged,
}
