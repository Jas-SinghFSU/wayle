use gtk::prelude::*;
use relm4::{gtk, prelude::*};

pub(super) struct DailyItem {
    pub day_label: String,
    pub icon_name: String,
    pub icon_color_class: &'static str,
    pub condition: String,
    pub high: String,
    pub low: String,
    pub bar_width: i32,
    pub bar_margin_start: i32,
    pub bar_fill_width: i32,
    pub is_today: bool,
}

#[relm4::factory(pub(super))]
impl FactoryComponent for DailyItem {
    type Init = DailyItem;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            #[watch]
            set_css_classes: &if self.is_today {
                vec!["daily-item", "today"]
            } else {
                vec!["daily-item"]
            },

            gtk::Label {
                add_css_class: "daily-day",
                set_halign: gtk::Align::Start,
                set_xalign: 0.0,
                #[watch]
                set_label: &self.day_label,
            },

            gtk::Image {
                #[watch]
                set_css_classes: &["daily-icon", self.icon_color_class],
                #[watch]
                set_icon_name: Some(&self.icon_name),
            },

            gtk::Label {
                add_css_class: "daily-condition",
                set_hexpand: true,
                set_halign: gtk::Align::Start,
                set_ellipsize: gtk::pango::EllipsizeMode::End,
                #[watch]
                set_label: &self.condition,
            },

            gtk::Box {
                add_css_class: "daily-bar",
                set_valign: gtk::Align::Center,
                #[watch]
                set_width_request: self.bar_width,

                gtk::Box {
                    add_css_class: "daily-bar-fill",
                    #[watch]
                    set_margin_start: self.bar_margin_start,
                    #[watch]
                    set_width_request: self.bar_fill_width,
                },
            },

            #[name = "temps"]
            gtk::Box {
                add_css_class: "daily-temps",

                gtk::Label {
                    add_css_class: "daily-high",
                    #[watch]
                    set_label: &self.high,
                },

                gtk::Label {
                    add_css_class: "daily-low",
                    #[watch]
                    set_label: &self.low,
                },
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        init
    }
}
