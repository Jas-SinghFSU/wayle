use relm4::ComponentSender;
use wayle_common::{services, watch};
use wayle_config::schemas::modules::WeatherConfig;
use wayle_weather::WeatherService;

use super::{
    WeatherModule,
    helpers::{FormatContext, condition_icon, convert_temp_unit, format_label},
    messages::WeatherCmd,
};

pub(super) fn spawn_watchers(sender: &ComponentSender<WeatherModule>, config: &WeatherConfig) {
    spawn_weather_watcher(sender, config);
}

fn spawn_weather_watcher(sender: &ComponentSender<WeatherModule>, config: &WeatherConfig) {
    let Some(weather_service) = services::try_get::<WeatherService>() else {
        return;
    };
    let weather_prop = weather_service.weather.clone();
    let units_config = config.units.clone();
    let format_config = config.format.clone();
    let fallback_icon = config.icon_name.clone();

    watch!(
        sender,
        [
            weather_prop.watch(),
            format_config.watch(),
            units_config.watch()
        ],
        |out| {
            let Some(weather) = weather_prop.get() else {
                let _ = out.send(WeatherCmd::UpdateIcon(fallback_icon.get().clone()));
                return;
            };

            let label = format_label(&FormatContext {
                format: &format_config.get(),
                weather: &weather,
                units: convert_temp_unit(units_config.get()),
            });
            let icon = condition_icon(weather.current.condition, weather.current.is_day);
            let _ = out.send(WeatherCmd::UpdateLabel(label));
            let _ = out.send(WeatherCmd::UpdateIcon(icon.to_string()));
        }
    );
}
