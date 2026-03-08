use std::{sync::Arc, time::Duration};

use relm4::ComponentSender;
use wayle_common::watch;
use wayle_config::ConfigService;
use wayle_weather::WeatherService;

use super::{WeatherHeader, messages::WeatherHeaderCmd};

const UPDATED_AGO_INTERVAL: Duration = Duration::from_secs(60);

pub(super) fn spawn(
    sender: &ComponentSender<WeatherHeader>,
    weather: &Arc<WeatherService>,
    config: &Arc<ConfigService>,
) {
    let weather_prop = weather.weather.clone();
    let units_config = config.config().modules.weather.units.clone();

    watch!(
        sender,
        [weather_prop.watch(), units_config.watch()],
        |out| {
            let _ = out.send(WeatherHeaderCmd::WeatherChanged);
        }
    );

    sender.command(|out, shutdown| async move {
        let shutdown_fut = shutdown.wait();
        tokio::pin!(shutdown_fut);

        loop {
            tokio::select! {
                () = &mut shutdown_fut => break,
                () = tokio::time::sleep(UPDATED_AGO_INTERVAL) => {
                    let _ = out.send(WeatherHeaderCmd::TickUpdatedAgo);
                }
            }
        }
    });
}
