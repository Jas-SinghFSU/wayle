use wayle_widgets::prelude::BarSettings;

pub(crate) struct WeatherInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub(crate) enum WeatherMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum WeatherCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
