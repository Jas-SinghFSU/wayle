use std::sync::Arc;

use relm4::ComponentSender;
use wayle_config::ConfigService;
use wayle_weather::WeatherService;
use wayle_widgets::watch;

use super::{SunTimes, messages::SunTimesCmd};

pub(super) fn spawn(
    sender: &ComponentSender<SunTimes>,
    weather: &Arc<WeatherService>,
    config: &Arc<ConfigService>,
) {
    let weather_prop = weather.weather.clone();
    let time_format = config.config().modules.weather.time_format.clone();

    watch!(sender, [weather_prop.watch(), time_format.watch()], |out| {
        let _ = out.send(SunTimesCmd::WeatherChanged);
    });
}
