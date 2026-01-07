use chrono::Utc;
use glib::Variant;
use gtk4::{
    PopoverMenu,
    gio::{
        Menu, MenuItem as GMenuItem, SimpleAction, SimpleActionGroup,
        prelude::{ActionMapExt, MenuModelExt},
    },
    prelude::WidgetExt,
};
use tracing::error;

use crate::{
    core::item::TrayItem,
    types::menu::{MenuEvent, MenuItem, MenuItemType, ToggleState, ToggleType},
};

/// GTK4 menu model components for a tray item.
#[derive(Debug)]
pub struct TrayMenuModel {
    /// The GTK Menu model representing the tray item's menu structure.
    pub menu: Menu,
    /// Action group containing all menu item actions.
    pub actions: SimpleActionGroup,
}

/// GTK4 adapter for system tray menus.
///
/// Converts SystemTray Service items into native GTK4 menu widgets.
pub struct Adapter;

impl Adapter {
    /// Builds a GTK GMenu model and action group from a tray item.
    ///
    /// Creates a menu structure with sections separated by separators,
    /// and registers actions for each menu item including checkboxes and radio buttons.
    pub fn build_model(tray_item: &TrayItem) -> TrayMenuModel {
        let menu = Menu::new();
        let actions = SimpleActionGroup::new();

        let Some(menu_item) = tray_item.menu.get() else {
            return TrayMenuModel { menu, actions };
        };

        Self::append_items_with_sections(&menu_item.children, &menu, &actions, tray_item);

        TrayMenuModel { menu, actions }
    }

    /// Builds a GTK PopoverMenu widget from a tray item.
    ///
    /// Creates a complete popover menu ready to display, with all actions
    /// configured and registered to the "app" action group.
    pub fn build_popover(tray_item: &TrayItem) -> PopoverMenu {
        let TrayMenuModel { menu, actions } = Self::build_model(tray_item);

        let popover = PopoverMenu::from_model(Some(&menu));
        popover.insert_action_group("app", Some(&actions));

        popover
    }

    fn add_to_menu(
        menu_item: &MenuItem,
        menu: &Menu,
        action_group: &SimpleActionGroup,
        tray_item: &TrayItem,
    ) {
        let label = menu_item
            .label
            .as_ref()
            .map(|l| l.trim_start_matches("_"))
            .unwrap_or("");

        if menu_item.has_children() {
            let submenu = Menu::new();
            Self::append_items_with_sections(
                &menu_item.children,
                &submenu,
                action_group,
                tray_item,
            );
            menu.append_submenu(Some(label), &submenu);

            return;
        }

        let action_name = format!("item_{}", &menu_item.id);
        let tray_item_clone = tray_item.clone();
        let id = menu_item.id;

        let menu_event_handler = move |_action: &SimpleAction, _param: Option<&Variant>| {
            let tray_item = tray_item_clone.clone();
            tokio::spawn(async move {
                if let Err(e) = tray_item
                    .menu_event(id, MenuEvent::Clicked, Utc::now().timestamp() as u32)
                    .await
                {
                    error!("Failed to send menu event: {e}");
                }
            });
        };

        match &menu_item.toggle_type {
            ToggleType::Checkmark | ToggleType::Radio => {
                let is_checked = menu_item.toggle_state == ToggleState::Checked;
                let action =
                    SimpleAction::new_stateful(&action_name, None, &Variant::from(&is_checked));

                action.connect_activate(menu_event_handler);
                action.set_enabled(menu_item.enabled);
                action_group.add_action(&action);
            }
            _ => {
                let action = SimpleAction::new(&action_name, None);

                action.connect_activate(menu_event_handler);
                action.set_enabled(menu_item.enabled);
                action_group.add_action(&action);
            }
        }

        let item = GMenuItem::new(Some(label), Some(&format!("app.{action_name}")));

        if let Some(shortcut) = &menu_item.shortcut
            && let Some(accel) = Self::to_gtk_accelerator(shortcut)
        {
            item.set_attribute_value("accel", Some(&Variant::from(accel)));
        }

        menu.append_item(&item);
    }

    fn append_items_with_sections(
        items: &[MenuItem],
        target_menu: &Menu,
        action_group: &SimpleActionGroup,
        tray_item: &TrayItem,
    ) {
        let mut section = Menu::new();

        for item in items {
            if !item.visible {
                continue;
            }

            if item.item_type != MenuItemType::Separator {
                Self::add_to_menu(item, &section, action_group, tray_item);
                continue;
            }

            if section.n_items() == 0 {
                continue;
            }

            target_menu.append_section(None, &section);
            section = Menu::new();
        }

        if section.n_items() > 0 {
            target_menu.append_section(None, &section);
        }
    }

    fn to_gtk_accelerator(shortcut: &[Vec<String>]) -> Option<String> {
        let keys = shortcut.first()?;
        let (key, modifiers) = keys.split_last()?;

        let mut result = modifiers
            .iter()
            .filter_map(|m| match m.as_str() {
                "Control" => Some("<Control>"),
                "Shift" => Some("<Shift>"),
                "Alt" => Some("<Alt>"),
                "Super" => Some("<Super>"),
                _ => None,
            })
            .collect::<String>();

        result.push_str(key);
        Some(result)
    }
}
