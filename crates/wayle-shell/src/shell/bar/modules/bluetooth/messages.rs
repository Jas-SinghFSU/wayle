use wayle_widgets::prelude::BarSettings;

pub struct BluetoothInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum BluetoothMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub enum BluetoothCmd {
    StateChanged,
    IconConfigChanged,
    AdapterChanged,
}
