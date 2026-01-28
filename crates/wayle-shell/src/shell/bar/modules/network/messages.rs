use wayle_widgets::prelude::BarSettings;

pub struct NetworkInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum NetworkMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum NetworkCmd {
    StateChanged,
    IconConfigChanged,
    WifiDeviceChanged,
    WiredDeviceChanged,
}
