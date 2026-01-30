use wayle_widgets::prelude::BarSettings;

pub(crate) struct PowerInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub(crate) enum PowerMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum PowerCmd {
    IconConfigChanged,
}
