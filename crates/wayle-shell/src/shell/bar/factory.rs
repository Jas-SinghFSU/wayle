//! Bar item factory for creating modules and groups.

use gtk::prelude::*;
use relm4::prelude::*;
use wayle_config::schemas::bar::BarItem;
use wayle_widgets::prelude::BarSettings;

use crate::shell::bar::modules::{ModuleController, create_module};

pub(crate) struct BarItemFactoryInit {
    pub(crate) item: BarItem,
    pub(crate) settings: BarSettings,
}

pub(crate) struct BarItemFactory {
    item: BarItem,
    settings: BarSettings,
    modules: Vec<ModuleController>,
}

#[relm4::factory(pub(crate))]
impl FactoryComponent for BarItemFactory {
    type Init = BarItemFactoryInit;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            add_css_class: "bar-item",
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let modules = match &init.item {
            BarItem::Module(module) => {
                vec![create_module(module, &init.settings)]
            }
            BarItem::Group(group) => group
                .modules
                .iter()
                .map(|m| create_module(m, &init.settings))
                .collect(),
        };

        Self {
            item: init.item,
            settings: init.settings,
            modules,
        }
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();

        let orientation = if self.settings.is_vertical.get() {
            gtk::Orientation::Vertical
        } else {
            gtk::Orientation::Horizontal
        };
        root.set_orientation(orientation);

        if let BarItem::Group(group) = &self.item {
            root.set_widget_name(&group.name);
            root.add_css_class("bar-group");
        }

        for controller in &self.modules {
            root.append(controller.widget());
        }

        widgets
    }
}
