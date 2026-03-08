mod helpers;
mod hourly_item;
mod messages;
mod methods;
mod watchers;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::{factory::FactoryVecDeque, gtk, prelude::*};
use wayle_config::ConfigService;
use wayle_weather::WeatherService;

pub(crate) use self::messages::HourlyForecastInit;
use self::{hourly_item::HourlyItem, messages::HourlyForecastCmd};
use crate::i18n::t;

pub(crate) struct HourlyForecast {
    weather: Arc<WeatherService>,
    config: Arc<ConfigService>,
    items: FactoryVecDeque<HourlyItem>,
}

#[relm4::component(pub(crate))]
impl Component for HourlyForecast {
    type Init = HourlyForecastInit;
    type Input = ();
    type Output = ();
    type CommandOutput = HourlyForecastCmd;

    view! {
        #[root]
        gtk::Box {
            add_css_class: "weather-section",
            set_orientation: gtk::Orientation::Vertical,

            gtk::Label {
                add_css_class: "section-label",
                set_halign: gtk::Align::Start,
                set_label: &t!("dropdown-weather-hourly"),
            },

            #[local_ref]
            forecast_row -> gtk::Box {
                add_css_class: "hourly-forecast",
                set_homogeneous: true,
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let items = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .detach();

        let mut model = Self {
            weather: init.weather.clone(),
            config: init.config.clone(),
            items,
        };

        model.refresh();
        watchers::spawn(&sender, &init.weather, &init.config);

        let forecast_row = model.items.widget();
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
            HourlyForecastCmd::WeatherChanged => {
                self.refresh();
            }
        }
    }
}
