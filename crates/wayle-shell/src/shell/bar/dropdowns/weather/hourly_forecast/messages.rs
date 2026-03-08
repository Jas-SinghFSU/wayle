use std::sync::Arc;

use wayle_config::ConfigService;
use wayle_weather::WeatherService;

pub(crate) struct HourlyForecastInit {
    pub weather: Arc<WeatherService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum HourlyForecastCmd {
    WeatherChanged,
}
