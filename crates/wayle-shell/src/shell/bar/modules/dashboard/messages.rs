use wayle_widgets::prelude::BarSettings;

pub(crate) struct DashboardInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub(crate) enum DashboardMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum DashboardCmd {
    IconConfigChanged,
}
