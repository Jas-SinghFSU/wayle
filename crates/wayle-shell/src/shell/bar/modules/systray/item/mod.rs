mod helpers;

use std::sync::Arc;

use gtk4::gio::SimpleActionGroup;
use helpers::{
    apply_icon_color, create_texture_from_pixmap, load_icon_from_theme_path, select_best_pixmap,
};
use relm4::{
    gtk::{self, prelude::*},
    prelude::*,
};
use tracing::debug;
use wayle_common::services;
use wayle_config::ConfigService;
use wayle_systray::{adapters::gtk4::Adapter, core::item::TrayItem, types::Coordinates};

use super::helpers::find_override;

pub(super) struct SystrayItemInit {
    pub(super) item: Arc<TrayItem>,
}

pub(super) struct SystrayItem {
    item: Arc<TrayItem>,
    button: Option<gtk::Button>,
    popover: Option<gtk::PopoverMenu>,
    action_group: Option<SimpleActionGroup>,
    registered_accels: Vec<String>,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(super) enum SystrayItemMsg {
    LeftClick,
    RightClick,
    MiddleClick,
}

#[derive(Debug)]
pub(super) enum SystrayItemOutput {}

#[relm4::factory(pub(super))]
impl FactoryComponent for SystrayItem {
    type Init = SystrayItemInit;
    type Input = SystrayItemMsg;
    type Output = SystrayItemOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Button {
            set_css_classes: &["systray-item"],
            set_cursor_from_name: Some("pointer"),

            #[name = "icon"]
            gtk::Image {},
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &relm4::factory::DynamicIndex,
        _sender: relm4::prelude::FactorySender<Self>,
    ) -> Self {
        Self {
            item: init.item,
            button: None,
            popover: None,
            action_group: None,
            registered_accels: Vec::new(),
        }
    }

    fn init_widgets(
        &mut self,
        _index: &relm4::factory::DynamicIndex,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: relm4::prelude::FactorySender<Self>,
    ) -> Self::Widgets {
        let item_id = self.item.id.get();
        root.set_widget_name(&item_id);
        debug!(item_id = %item_id, "init_widgets: setting up button");

        self.button = Some(root.clone());

        let left_click = gtk::GestureClick::builder().button(1).build();
        let right_click = gtk::GestureClick::builder().button(3).build();
        let middle_click = gtk::GestureClick::builder().button(2).build();

        left_click.connect_released({
            let sender = sender.clone();
            move |gesture, _, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
                sender.input(SystrayItemMsg::LeftClick);
            }
        });

        right_click.connect_released({
            let sender = sender.clone();
            move |gesture, _, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
                sender.input(SystrayItemMsg::RightClick);
            }
        });

        middle_click.connect_released({
            let sender = sender.clone();
            move |gesture, _, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
                sender.input(SystrayItemMsg::MiddleClick);
            }
        });

        root.add_controller(left_click);
        root.add_controller(right_click);
        root.add_controller(middle_click);

        let widgets = view_output!();

        self.update_icon(&widgets.icon);

        widgets
    }

    fn update(&mut self, msg: Self::Input, _sender: relm4::prelude::FactorySender<Self>) {
        match msg {
            SystrayItemMsg::LeftClick => {
                let item = self.item.clone();
                tokio::spawn(async move {
                    if item.item_is_menu.get() {
                        let _ = item.context_menu(Coordinates::new(0, 0)).await;
                    } else {
                        let _ = item.activate(Coordinates::new(0, 0)).await;
                    }
                });
            }
            SystrayItemMsg::RightClick => {
                self.toggle_menu();
            }
            SystrayItemMsg::MiddleClick => {
                let item = self.item.clone();
                tokio::spawn(async move {
                    let _ = item.secondary_activate(Coordinates::new(0, 0)).await;
                });
            }
        }
    }
}

impl SystrayItem {
    fn toggle_menu(&mut self) {
        if let Some(popover) = self.popover.as_ref()
            && popover.is_visible()
        {
            debug!(item_id = %self.item.id.get(), "hiding popover");
            popover.popdown();
            return;
        }

        self.show_menu();
    }

    fn show_menu(&mut self) {
        let item_id = self.item.id.get();
        debug!(item_id = %item_id, title = %self.item.title.get(), "show_menu called");

        let menu_data = self.item.menu.get();
        let Some(root_menu) = menu_data else {
            debug!("no menu data, falling back");
            self.spawn_context_menu_fallback();
            return;
        };

        if root_menu.children.is_empty() {
            debug!("empty menu, falling back");
            self.spawn_context_menu_fallback();
            return;
        }

        let model = Adapter::build_model(&self.item);
        debug!(
            item_id = %item_id,
            menu_n_items = model.menu.n_items(),
            accelerators = model.accelerators.len(),
            "built menu model"
        );

        self.clear_accelerators();

        if let Some(popover) = self.popover.clone() {
            popover.set_menu_model(Some(&model.menu));
            popover.insert_action_group("app", Some(&model.actions));
            self.action_group = Some(model.actions);
            self.register_accelerators(&popover, &model.accelerators);
            popover.popup();
        } else {
            let popover =
                gtk::PopoverMenu::from_model_full(&model.menu, gtk::PopoverMenuFlags::NESTED);
            popover.add_css_class("systray-menu");
            popover.insert_action_group("app", Some(&model.actions));
            popover.set_has_arrow(false);

            if let Some(parent) = self.button.as_ref() {
                popover.set_parent(parent);
            }

            self.register_accelerators(&popover, &model.accelerators);

            self.action_group = Some(model.actions);
            self.popover = Some(popover.clone());
            popover.popup();
        }
    }

    fn register_accelerators(
        &mut self,
        popover: &gtk::PopoverMenu,
        accelerators: &[(String, String)],
    ) {
        let Some(app) = popover
            .root()
            .and_then(|r| r.downcast::<gtk::Window>().ok())
            .and_then(|w| w.application())
        else {
            return;
        };

        for (action_name, accel) in accelerators {
            app.set_accels_for_action(action_name, &[accel.as_str()]);
            self.registered_accels.push(action_name.clone());
        }
    }

    fn clear_accelerators(&mut self) {
        let Some(popover) = self.popover.as_ref() else {
            return;
        };

        let Some(app) = popover
            .root()
            .and_then(|r| r.downcast::<gtk::Window>().ok())
            .and_then(|w| w.application())
        else {
            return;
        };

        for action_name in self.registered_accels.drain(..) {
            app.set_accels_for_action(&action_name, &[]);
        }
    }

    fn spawn_context_menu_fallback(&self) {
        let item = self.item.clone();
        tokio::spawn(async move {
            let _ = item.context_menu(Coordinates::new(0, 0)).await;
        });
    }

    fn update_icon(&self, image: &gtk::Image) {
        let config_service = services::get::<ConfigService>();
        let overrides = config_service.config().modules.systray.overrides.get();
        let override_match = find_override(&self.item, &overrides);

        let icon_name = override_match
            .and_then(|m| m.icon.clone())
            .or_else(|| self.item.icon_name.get());

        if let Some(ref name) = icon_name {
            let theme_path = self.item.icon_theme_path.get();
            let texture = theme_path
                .as_deref()
                .and_then(|path| load_icon_from_theme_path(path, name));

            if let Some(texture) = texture {
                image.set_paintable(Some(&texture));
            } else {
                image.set_icon_name(Some(name));
            }
        } else {
            let pixmaps = self.item.icon_pixmap.get();
            let texture = select_best_pixmap(&pixmaps).and_then(create_texture_from_pixmap);

            if let Some(texture) = texture {
                image.set_paintable(Some(&texture));
            } else {
                image.set_icon_name(Some("application-x-executable-symbolic"));
            }
        }

        if let Some(color) = override_match.and_then(|m| m.color.clone()) {
            apply_icon_color(image, &color.to_css());
        }
    }
}
