use wayle_widgets::prelude::BarSettings;

pub struct BatteryInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum BatteryMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub enum BatteryCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
