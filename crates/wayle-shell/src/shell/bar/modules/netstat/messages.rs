use wayle_widgets::prelude::BarSettings;

pub(crate) struct NetstatInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub(crate) enum NetstatMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum NetstatCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
