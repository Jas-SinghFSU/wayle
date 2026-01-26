mod helpers;
mod item;
mod messages;
mod styling;
mod watchers;

use std::sync::Arc;

use gtk4::prelude::{OrientableExt, WidgetExt};
use helpers::is_blacklisted;
use item::{SystrayItem, SystrayItemInit};
pub(crate) use messages::{SystrayCmd, SystrayInit, SystrayMsg};
use relm4::{ComponentParts, ComponentSender, factory::FactoryVecDeque, gtk, prelude::*};
use wayle_common::{ConfigProperty, services};
use wayle_config::ConfigService;
use wayle_systray::core::item::TrayItem;
use wayle_widgets::prelude::{
    BarContainer, BarContainerBehavior, BarContainerColors, BarContainerInit, force_window_resize,
};

pub(crate) struct SystrayModule {
    container: Controller<BarContainer>,
    items: FactoryVecDeque<SystrayItem>,
    css_provider: gtk::CssProvider,
    visible: ConfigProperty<bool>,
}

#[relm4::component(pub(crate))]
impl Component for SystrayModule {
    type Init = SystrayInit;
    type Input = SystrayMsg;
    type Output = ();
    type CommandOutput = SystrayCmd;

    view! {
        gtk::Box {
            #[local_ref]
            container -> gtk::Box {
                #[local_ref]
                items_box -> gtk::Box {},
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config_service = services::get::<ConfigService>();
        let config = &config_service.config().modules.systray;
        let styling_config = &config_service.config().styling;
        let bar_config = &config_service.config().bar;

        let visible = ConfigProperty::new(false);

        let container = BarContainer::builder()
            .launch(BarContainerInit {
                colors: BarContainerColors {
                    background: config.button_bg_color.clone(),
                    border_color: config.border_color.clone(),
                },
                behavior: BarContainerBehavior {
                    show_border: config.border_show.clone(),
                    visible: visible.clone(),
                },
                is_vertical: init.is_vertical.clone(),
                theme_provider: styling_config.theme_provider.clone(),
                border_width: bar_config.button_border_width.clone(),
                border_location: bar_config.button_border_location.clone(),
            })
            .detach();

        let orientation = if init.is_vertical.get() {
            gtk::Orientation::Vertical
        } else {
            gtk::Orientation::Horizontal
        };
        let items = FactoryVecDeque::builder()
            .launch(gtk::Box::new(orientation, 0))
            .detach();

        let css_provider = styling::init_css_provider(items.widget());

        watchers::spawn_watchers(&sender, &init.is_vertical);

        let model = Self {
            container,
            items,
            css_provider,
            visible,
        };
        let container = model.container.widget();
        let items_box = model.items.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_cmd(
        &mut self,
        msg: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match msg {
            SystrayCmd::ItemsChanged(items) => {
                self.update_items(items);
                if let Some(parent) = root.parent() {
                    parent.set_visible(self.visible.get());
                }
                force_window_resize(root);
            }
            SystrayCmd::StylingChanged => {
                styling::reload_css(&self.css_provider);
                force_window_resize(root);
            }
            SystrayCmd::OrientationChanged(vertical) => {
                let orientation = if vertical {
                    gtk::Orientation::Vertical
                } else {
                    gtk::Orientation::Horizontal
                };
                self.items.widget().set_orientation(orientation);
                force_window_resize(root);
            }
        }
    }
}

impl SystrayModule {
    fn update_items(&mut self, items: Vec<Arc<TrayItem>>) {
        let config_service = services::get::<ConfigService>();
        let config = &config_service.config().modules.systray;

        let mut guard = self.items.guard();
        guard.clear();

        for item in items {
            if is_blacklisted(&item, config) {
                continue;
            }
            guard.push_back(SystrayItemInit { item });
        }

        self.visible.set(!guard.is_empty());
    }
}
