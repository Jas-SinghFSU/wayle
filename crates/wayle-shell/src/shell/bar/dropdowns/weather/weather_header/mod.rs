mod helpers;
mod messages;
mod methods;
mod watchers;

use std::sync::Arc;

use gtk::{pango, prelude::*};
use relm4::{gtk, prelude::*};
use wayle_config::ConfigService;
use wayle_weather::WeatherService;

use self::messages::WeatherHeaderCmd;
pub(crate) use self::messages::WeatherHeaderInit;

pub(crate) struct WeatherHeader {
    weather: Arc<WeatherService>,
    config: Arc<ConfigService>,

    icon_name: String,
    icon_color_class: &'static str,
    temp_value: String,
    temp_unit: &'static str,
    condition: String,
    location: String,
    updated_ago: String,
}

#[relm4::component(pub(crate))]
impl Component for WeatherHeader {
    type Init = WeatherHeaderInit;
    type Input = ();
    type Output = ();
    type CommandOutput = WeatherHeaderCmd;

    view! {
        #[root]
        gtk::Box {
            add_css_class: "weather-header",

            #[name = "current"]
            gtk::Box {
                add_css_class: "weather-current",

                gtk::Image {
                    add_css_class: "weather-icon",
                    #[watch]
                    set_css_classes: &["weather-icon", model.icon_color_class()],
                    #[watch]
                    set_icon_name: Some(model.icon_name()),
                },

                #[name = "temp_group"]
                gtk::Box {
                    add_css_class: "weather-temp-group",
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,

                    #[name = "temp_row"]
                    gtk::Box {
                        set_halign: gtk::Align::Start,
                        set_valign: gtk::Align::Baseline,
                        set_spacing: 2,

                        gtk::Label {
                            add_css_class: "weather-temp",
                            #[watch]
                            set_label: model.temp_value(),
                        },

                        gtk::Label {
                            add_css_class: "weather-temp-unit",
                            set_valign: gtk::Align::Baseline,
                            #[watch]
                            set_label: model.temp_unit(),
                        },
                    },

                    gtk::Label {
                        add_css_class: "weather-condition",
                        set_halign: gtk::Align::Start,
                        #[watch]
                        set_label: model.condition(),
                    },
                },
            },

            #[name = "location_info"]
            gtk::Box {
                add_css_class: "weather-location",
                set_orientation: gtk::Orientation::Vertical,
                set_valign: gtk::Align::Center,
                set_hexpand: true,
                set_halign: gtk::Align::End,

                gtk::Label {
                    add_css_class: "weather-city",
                    set_halign: gtk::Align::End,
                    set_ellipsize: pango::EllipsizeMode::End,
                    #[watch]
                    set_label: model.location(),
                },

                gtk::Label {
                    add_css_class: "weather-updated",
                    set_halign: gtk::Align::End,
                    #[watch]
                    set_label: model.updated_ago(),
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
            icon_name: String::from("ld-sun-symbolic"),
            icon_color_class: "sunny",
            temp_value: String::new(),
            temp_unit: "",
            condition: String::new(),
            location: String::new(),
            updated_ago: String::new(),
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
            WeatherHeaderCmd::WeatherChanged => {
                self.refresh();
            }

            WeatherHeaderCmd::TickUpdatedAgo => {
                self.refresh_updated_ago();
            }
        }
    }
}
