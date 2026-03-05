use std::sync::Arc;

use relm4::ComponentSender;
use wayle_common::watch;
use wayle_config::ConfigService;
use wayle_weather::WeatherService;

use super::{HourlyForecast, messages::HourlyForecastCmd};

pub(super) fn spawn(
    sender: &ComponentSender<HourlyForecast>,
    weather: &Arc<WeatherService>,
    config: &Arc<ConfigService>,
) {
    let weather_prop = weather.weather.clone();
    let units_config = config.config().modules.weather.units.clone();
    let time_format = config.config().modules.weather.time_format.clone();

    watch!(
        sender,
        [
            weather_prop.watch(),
            units_config.watch(),
            time_format.watch()
        ],
        |out| {
            let _ = out.send(HourlyForecastCmd::WeatherChanged);
        }
    );
}
