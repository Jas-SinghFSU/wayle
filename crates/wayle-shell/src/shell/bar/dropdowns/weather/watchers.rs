use std::sync::Arc;

use relm4::ComponentSender;
use wayle_config::ConfigService;
use wayle_weather::{WeatherService, WeatherStatus};
use wayle_widgets::watch;

use super::{
    WeatherDropdown,
    messages::{WeatherDropdownCmd, WeatherPage},
};

pub(super) fn spawn(
    sender: &ComponentSender<WeatherDropdown>,
    weather: &Arc<WeatherService>,
    config: &Arc<ConfigService>,
) {
    spawn_scale_watcher(sender, config);
    spawn_page_watcher(sender, weather);
}

fn spawn_scale_watcher(sender: &ComponentSender<WeatherDropdown>, config: &Arc<ConfigService>) {
    let scale = config.config().styling.scale.clone();

    watch!(sender, [scale.watch()], |out| {
        let _ = out.send(WeatherDropdownCmd::ScaleChanged(scale.get().value()));
    });
}

fn spawn_page_watcher(sender: &ComponentSender<WeatherDropdown>, weather: &Arc<WeatherService>) {
    let status_prop = weather.status.clone();

    watch!(sender, [status_prop.watch()], |out| {
        let status = status_prop.get();
        let (page, error) = match &status {
            WeatherStatus::Loading => (WeatherPage::Loading, None),
            WeatherStatus::Loaded => (WeatherPage::Loaded, None),
            WeatherStatus::Error(kind) => (WeatherPage::Error, Some(kind.clone())),
        };
        let _ = out.send(WeatherDropdownCmd::PageChanged { page, error });
    });
}
