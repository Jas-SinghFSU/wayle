use wayle_widgets::prelude::BarSettings;

pub(crate) struct WorldClockInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub(crate) enum WorldClockMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum WorldClockCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
