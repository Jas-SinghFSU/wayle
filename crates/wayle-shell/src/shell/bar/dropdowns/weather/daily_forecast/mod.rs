mod daily_item;
mod helpers;
mod messages;
mod methods;
mod watchers;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::{factory::FactoryVecDeque, gtk, prelude::*};
use wayle_config::ConfigService;
use wayle_weather::WeatherService;

pub(crate) use self::messages::DailyForecastInit;
use self::{daily_item::DailyItem, messages::DailyForecastCmd};
use crate::i18n::t;

pub(crate) struct DailyForecast {
    weather: Arc<WeatherService>,
    config: Arc<ConfigService>,
    items: FactoryVecDeque<DailyItem>,
}

#[relm4::component(pub(crate))]
impl Component for DailyForecast {
    type Init = DailyForecastInit;
    type Input = ();
    type Output = ();
    type CommandOutput = DailyForecastCmd;

    view! {
        #[root]
        gtk::Box {
            add_css_class: "weather-section",
            set_orientation: gtk::Orientation::Vertical,

            gtk::Label {
                add_css_class: "section-label",
                set_halign: gtk::Align::Start,
                set_label: &t!("dropdown-weather-daily"),
            },

            #[local_ref]
            forecast_list -> gtk::Box {
                add_css_class: "daily-forecast",
                set_orientation: gtk::Orientation::Vertical,
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let items = FactoryVecDeque::builder()
            .launch(gtk::Box::new(gtk::Orientation::Vertical, 0))
            .detach();

        let mut model = Self {
            weather: init.weather.clone(),
            config: init.config.clone(),
            items,
        };

        model.refresh();
        watchers::spawn(&sender, &init.weather, &init.config);

        let forecast_list = model.items.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update_cmd(
        &mut self,
        msg: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            DailyForecastCmd::WeatherChanged => {
                self.refresh();
            }
        }
    }
}
