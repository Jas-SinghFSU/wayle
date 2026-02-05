use wayle_widgets::prelude::BarSettings;

use super::helpers::HyprsunsetState;

pub(crate) struct HyprsunsetInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub(crate) enum HyprsunsetMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum HyprsunsetCmd {
    ConfigChanged,
    StateChanged(Option<HyprsunsetState>),
}
