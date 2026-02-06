use std::sync::Arc;

use wayle_config::ConfigService;
use wayle_weather::WeatherService;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct WeatherInit {
    pub settings: BarSettings,
    pub weather: Arc<WeatherService>,
    pub config: Arc<ConfigService>,
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
