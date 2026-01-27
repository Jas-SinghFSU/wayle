use wayle_widgets::prelude::BarSettings;

pub struct DashboardInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum DashboardMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub enum DashboardCmd {
    IconConfigChanged,
}
