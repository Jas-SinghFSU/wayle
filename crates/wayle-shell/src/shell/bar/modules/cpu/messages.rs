use wayle_widgets::prelude::BarSettings;

pub(crate) struct CpuInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub(crate) enum CpuMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum CpuCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
