use wayle_widgets::prelude::BarSettings;

pub(crate) struct ClockInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub(crate) enum ClockMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum ClockCmd {
    UpdateTime(String),
    UpdateIcon(String),
    UpdateTooltip(Option<String>),
}
