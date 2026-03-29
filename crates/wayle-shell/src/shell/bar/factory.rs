//! Bar item factory for creating modules and groups.

use std::rc::Rc;

use gtk::prelude::*;
use relm4::prelude::*;
use wayle_config::schemas::bar::BarItem;
use wayle_widgets::prelude::BarSettings;

use crate::shell::{
    bar::{
        dropdowns::DropdownRegistry,
        modules::{ModuleInstance, create_module},
    },
    services::ShellServices,
};

pub(crate) struct BarItemFactoryInit {
    pub(crate) item: BarItem,
    pub(crate) settings: BarSettings,
    pub(crate) services: ShellServices,
    pub(crate) dropdowns: Rc<DropdownRegistry>,
}

pub(crate) struct BarItemFactory {
    item: BarItem,
    settings: BarSettings,
    #[allow(dead_code)]
    services: ShellServices,
    #[allow(dead_code)]
    dropdowns: Rc<DropdownRegistry>,
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
            BarItem::Module(module) => {
                create_module(module, &init.settings, &init.services, &init.dropdowns)
                    .into_iter()
                    .collect()
            }
            BarItem::Group(group) => group
                .modules
                .iter()
                .filter_map(|module| {
                    create_module(module, &init.settings, &init.services, &init.dropdowns)
                })
                .collect(),
        };

        Self {
            item: init.item,
            settings: init.settings,
            services: init.services,
            dropdowns: init.dropdowns,
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

        for instance in &self.modules {
            let widget = instance.controller.widget();
            widget.add_css_class("module");
            if let Some(class) = &instance.class {
                widget.add_css_class(class);
            }
            root.append(widget);

            let container = root.clone();
            widget.connect_notify_local(Some("visible"), move |_, _| {
                sync_container_visibility(&container);
            });
        }

        sync_container_visibility(&root);

        widgets
    }
}

impl BarItemFactory {
    pub(crate) fn matches(&self, item: &BarItem) -> bool {
        self.item == *item
    }
}

fn sync_container_visibility(container: &gtk::Box) {
    let has_visible_child = container
        .observe_children()
        .into_iter()
        .filter_map(|obj| obj.ok()?.downcast::<gtk::Widget>().ok())
        .any(|widget| widget.get_visible());

    container.set_visible(has_visible_child);
}
