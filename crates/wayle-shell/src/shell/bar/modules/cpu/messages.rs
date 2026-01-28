use wayle_widgets::prelude::BarSettings;

pub struct CpuInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum CpuMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub enum CpuCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
