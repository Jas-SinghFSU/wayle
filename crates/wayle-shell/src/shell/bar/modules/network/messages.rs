use wayle_widgets::prelude::BarSettings;

pub(crate) struct NetworkInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub(crate) enum NetworkMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum NetworkCmd {
    StateChanged,
    IconConfigChanged,
    WifiDeviceChanged,
    WiredDeviceChanged,
}
