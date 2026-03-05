use std::sync::Arc;

use wayle_config::ConfigService;
use wayle_weather::{WeatherErrorKind, WeatherService};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WeatherPage {
    Loading,
    Loaded,
    Error,
}

impl WeatherPage {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Loading => "loading",
            Self::Loaded => "loaded",
            Self::Error => "error",
        }
    }
}

pub(crate) struct WeatherDropdownInit {
    pub weather: Arc<WeatherService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum WeatherDropdownInput {
    Retry,
}

#[derive(Debug)]
pub(crate) enum WeatherDropdownCmd {
    ScaleChanged(f32),
    PageChanged {
        page: WeatherPage,
        error: Option<WeatherErrorKind>,
    },
}
