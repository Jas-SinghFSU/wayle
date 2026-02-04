use wayle_widgets::prelude::BarSettings;

pub(crate) struct IdleInhibitInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub(crate) enum IdleInhibitMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum IdleInhibitCmd {
    ConfigChanged,
    StateChanged,
}
