use wayle_widgets::prelude::BarSettings;

pub(crate) struct BatteryInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub(crate) enum BatteryMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum BatteryCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
