use wayle_widgets::prelude::BarSettings;

pub struct NetworkInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum NetworkMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub enum NetworkCmd {
    StateChanged,
    IconConfigChanged,
}
