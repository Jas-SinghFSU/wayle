use wayle_widgets::prelude::BarSettings;

pub struct PowerInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum PowerMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub enum PowerCmd {
    IconConfigChanged,
}
