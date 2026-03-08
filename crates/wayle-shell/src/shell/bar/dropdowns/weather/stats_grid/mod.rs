mod messages;
mod methods;
mod watchers;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_config::ConfigService;
use wayle_weather::WeatherService;

use self::messages::StatsGridCmd;
pub(crate) use self::messages::StatsGridInit;
use crate::i18n::t;

pub(crate) struct StatsGrid {
    weather: Arc<WeatherService>,
    config: Arc<ConfigService>,

    humidity: String,
    wind: String,
    uv_index: String,
    rain_chance: String,
}

#[relm4::component(pub(crate))]
impl Component for StatsGrid {
    type Init = StatsGridInit;
    type Input = ();
    type Output = ();
    type CommandOutput = StatsGridCmd;

    view! {
        #[root]
        gtk::Box {
            set_css_classes: &["weather-stats"],
            set_hexpand: true,
            set_homogeneous: true,

            #[name = "humidity_stat"]
            gtk::Box {
                set_css_classes: &["weather-stat", "stat-first"],
                set_orientation: gtk::Orientation::Vertical,

                gtk::Image {
                    set_css_classes: &["weather-stat-icon", "humidity"],
                    set_icon_name: Some("ld-droplets-symbolic"),
                },
                gtk::Label {
                    add_css_class: "weather-stat-value",
                    #[watch]
                    set_label: model.humidity(),
                },
                gtk::Label {
                    add_css_class: "weather-stat-label",
                    set_label: &t!("dropdown-weather-humidity"),
                },
            },

            #[name = "wind_stat"]
            gtk::Box {
                add_css_class: "weather-stat",
                set_orientation: gtk::Orientation::Vertical,

                gtk::Image {
                    set_css_classes: &["weather-stat-icon", "wind"],
                    set_icon_name: Some("ld-wind-symbolic"),
                },
                gtk::Label {
                    add_css_class: "weather-stat-value",
                    #[watch]
                    set_label: model.wind(),
                },
                gtk::Label {
                    add_css_class: "weather-stat-label",
                    set_label: &t!("dropdown-weather-wind"),
                },
            },

            #[name = "uv_stat"]
            gtk::Box {
                add_css_class: "weather-stat",
                set_orientation: gtk::Orientation::Vertical,

                gtk::Image {
                    set_css_classes: &["weather-stat-icon", "uv"],
                    set_icon_name: Some("ld-sun-symbolic"),
                },
                gtk::Label {
                    add_css_class: "weather-stat-value",
                    #[watch]
                    set_label: model.uv_index(),
                },
                gtk::Label {
                    add_css_class: "weather-stat-label",
                    set_label: &t!("dropdown-weather-uv"),
                },
            },

            #[name = "rain_stat"]
            gtk::Box {
                set_css_classes: &["weather-stat", "stat-last"],
                set_orientation: gtk::Orientation::Vertical,

                gtk::Image {
                    set_css_classes: &["weather-stat-icon", "rain"],
                    set_icon_name: Some("ld-cloud-rain-symbolic"),
                },
                gtk::Label {
                    add_css_class: "weather-stat-value",
                    #[watch]
                    set_label: model.rain_chance(),
                },
                gtk::Label {
                    add_css_class: "weather-stat-label",
                    set_label: &t!("dropdown-weather-rain"),
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = Self {
            weather: init.weather.clone(),
            config: init.config.clone(),
            humidity: String::new(),
            wind: String::new(),
            uv_index: String::new(),
            rain_chance: String::new(),
        };

        model.refresh();
        watchers::spawn(&sender, &init.weather, &init.config);

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
            StatsGridCmd::WeatherChanged => {
                self.refresh();
            }
        }
    }
}
