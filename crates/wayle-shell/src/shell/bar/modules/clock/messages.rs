use wayle_widgets::prelude::BarSettings;

pub struct ClockInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum ClockMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum ClockCmd {
    UpdateTime(String),
    UpdateIcon(String),
    UpdateTooltip(Option<String>),
}
