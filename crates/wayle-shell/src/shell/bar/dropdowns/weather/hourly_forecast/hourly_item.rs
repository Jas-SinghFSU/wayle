use gtk::prelude::*;
use relm4::{gtk, prelude::*};

pub(super) struct HourlyItem {
    pub time_label: String,
    pub icon_name: String,
    pub icon_color_class: &'static str,
    pub temp_value: String,
}

#[relm4::factory(pub(super))]
impl FactoryComponent for HourlyItem {
    type Init = HourlyItem;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            add_css_class: "hourly-item",

            gtk::Label {
                add_css_class: "hourly-time",
                #[watch]
                set_label: &self.time_label,
            },

            gtk::Image {
                #[watch]
                set_css_classes: &["hourly-icon", self.icon_color_class],
                #[watch]
                set_icon_name: Some(&self.icon_name),
            },

            gtk::Label {
                add_css_class: "hourly-temp",
                #[watch]
                set_label: &self.temp_value,
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        init
    }
}
