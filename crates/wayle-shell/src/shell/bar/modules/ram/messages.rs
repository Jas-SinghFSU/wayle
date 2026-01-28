use wayle_widgets::prelude::BarSettings;

pub struct RamInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum RamMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub enum RamCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
