//! Bar item factory for creating modules and groups.

use gtk::prelude::*;
use relm4::prelude::*;
use wayle_config::schemas::bar::BarItem;
use wayle_widgets::prelude::BarSettings;

use crate::shell::{
    bar::modules::{ModuleInstance, create_module},
    services::ShellServices,
};

pub(crate) struct BarItemFactoryInit {
    pub(crate) item: BarItem,
    pub(crate) settings: BarSettings,
    pub(crate) services: ShellServices,
}

pub(crate) struct BarItemFactory {
    item: BarItem,
    settings: BarSettings,
    #[allow(dead_code)]
    services: ShellServices,
    modules: Vec<ModuleInstance>,
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
            BarItem::Module(module) => create_module(module, &init.settings, &init.services)
                .into_iter()
                .collect(),
            BarItem::Group(group) => group
                .modules
                .iter()
                .filter_map(|m| create_module(m, &init.settings, &init.services))
                .collect(),
        };

        Self {
            item: init.item,
            settings: init.settings,
            services: init.services,
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

        if self.modules.is_empty() {
            root.set_visible(false);
        }

        for instance in &self.modules {
            let widget = instance.controller.widget();
            widget.add_css_class("module");
            if let Some(class) = &instance.class {
                widget.add_css_class(class);
            }
            root.append(widget);
        }

        widgets
    }
}
