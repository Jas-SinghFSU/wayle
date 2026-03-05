mod helpers;
mod messages;
mod methods;
mod watchers;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_config::ConfigService;
use wayle_weather::WeatherService;

use self::messages::SunTimesCmd;
pub(crate) use self::messages::SunTimesInit;
use crate::i18n::t;

pub(crate) struct SunTimes {
    weather: Arc<WeatherService>,
    config: Arc<ConfigService>,
    sunrise: String,
    sunset: String,
}

#[relm4::component(pub(crate))]
impl Component for SunTimes {
    type Init = SunTimesInit;
    type Input = ();
    type Output = ();
    type CommandOutput = SunTimesCmd;

    view! {
        #[root]
        gtk::Box {
            add_css_class: "sun-times",

            #[name = "sunrise_section"]
            gtk::Box {
                add_css_class: "sun-time",
                set_hexpand: true,
                set_halign: gtk::Align::Start,

                gtk::Image {
                    set_css_classes: &["sun-icon", "sunrise"],
                    set_icon_name: Some("ld-sunrise-symbolic"),
                },

                #[name = "sunrise_info"]
                gtk::Box {
                    add_css_class: "sun-info",
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Label {
                        add_css_class: "sun-label",
                        set_halign: gtk::Align::Start,
                        set_label: &t!("dropdown-weather-sunrise"),
                    },

                    gtk::Label {
                        add_css_class: "sun-value",
                        set_halign: gtk::Align::Start,
                        #[watch]
                        set_label: model.sunrise(),
                    },
                },
            },

            #[name = "sunset_section"]
            gtk::Box {
                add_css_class: "sun-time",
                set_hexpand: true,
                set_halign: gtk::Align::End,

                gtk::Image {
                    set_css_classes: &["sun-icon", "sunset"],
                    set_icon_name: Some("ld-sunset-symbolic"),
                },

                #[name = "sunset_info"]
                gtk::Box {
                    add_css_class: "sun-info",
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Label {
                        add_css_class: "sun-label",
                        set_halign: gtk::Align::Start,
                        set_label: &t!("dropdown-weather-sunset"),
                    },

                    gtk::Label {
                        add_css_class: "sun-value",
                        set_halign: gtk::Align::Start,
                        #[watch]
                        set_label: model.sunset(),
                    },
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
            sunrise: String::new(),
            sunset: String::new(),
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
            SunTimesCmd::WeatherChanged => {
                self.refresh();
            }
        }
    }
}
