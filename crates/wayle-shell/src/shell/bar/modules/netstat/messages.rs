use wayle_widgets::prelude::BarSettings;

pub struct NetstatInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum NetstatMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub enum NetstatCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
