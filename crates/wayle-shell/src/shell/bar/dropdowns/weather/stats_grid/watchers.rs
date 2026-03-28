use std::sync::Arc;

use relm4::ComponentSender;
use wayle_config::ConfigService;
use wayle_weather::WeatherService;
use wayle_widgets::watch;

use super::{StatsGrid, messages::StatsGridCmd};

pub(super) fn spawn(
    sender: &ComponentSender<StatsGrid>,
    weather: &Arc<WeatherService>,
    config: &Arc<ConfigService>,
) {
    let weather_prop = weather.weather.clone();
    let units_config = config.config().modules.weather.units.clone();

    watch!(
        sender,
        [weather_prop.watch(), units_config.watch()],
        |out| {
            let _ = out.send(StatsGridCmd::WeatherChanged);
        }
    );
}
