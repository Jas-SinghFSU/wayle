use std::{
    collections::HashMap,
    fmt::{self, Debug, Display, Formatter},
};

use zbus::zvariant::OwnedValue;

/// Raw menu item properties from D-Bus.
/// (item_id, properties)
pub type RawMenuItemProps = (i32, HashMap<String, OwnedValue>);

/// Collection of menu items with properties.
pub type RawMenuItemsPropsList = Vec<RawMenuItemProps>;

/// Raw menu item property names to remove.
/// (item_id, property_names)
pub type RawMenuItemKeys = (i32, Vec<String>);

/// Collection of menu items with property names to remove.
pub type RawMenuItemKeysList = Vec<RawMenuItemKeys>;

/// Raw menu layout data from D-Bus GetLayout method.
/// (revision, (item_id, properties, children))
pub type RawMenuLayout = (u32, (i32, HashMap<String, OwnedValue>, Vec<OwnedValue>));

/// Type of a DBusMenu item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItemType {
    /// Standard clickable menu item.
    Standard,
    /// Menu separator.
    Separator,
}

impl Default for MenuItemType {
    fn default() -> Self {
        Self::Standard
    }
}

impl From<&str> for MenuItemType {
    fn from(s: &str) -> Self {
        match s {
            "separator" => Self::Separator,
            _ => Self::Standard,
        }
    }
}

impl Display for MenuItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "standard"),
            Self::Separator => write!(f, "separator"),
        }
    }
}

/// Toggle type for checkable menu items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleType {
    /// No toggle capability.
    None,
    /// Checkbox (independent toggle).
    Checkmark,
    /// Radio button (mutually exclusive within group).
    Radio,
}

impl Default for ToggleType {
    fn default() -> Self {
        Self::None
    }
}

impl From<&str> for ToggleType {
    fn from(s: &str) -> Self {
        match s {
            "checkmark" => Self::Checkmark,
            "radio" => Self::Radio,
            _ => Self::None,
        }
    }
}

impl Display for ToggleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::Checkmark => write!(f, "checkmark"),
            Self::Radio => write!(f, "radio"),
        }
    }
}

/// Toggle state for checkable menu items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleState {
    /// Unchecked state.
    Unchecked,
    /// Checked state.
    Checked,
    /// Indeterminate state.
    Unknown,
}

impl Default for ToggleState {
    fn default() -> Self {
        Self::Unchecked
    }
}

impl From<i32> for ToggleState {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Unchecked,
            1 => Self::Checked,
            _ => Self::Unknown,
        }
    }
}

impl From<ToggleState> for i32 {
    fn from(state: ToggleState) -> Self {
        match state {
            ToggleState::Unchecked => 0,
            ToggleState::Checked => 1,
            ToggleState::Unknown => -1,
        }
    }
}

/// Disposition of a menu item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    /// Normal menu item.
    Normal,
    /// Informative item.
    Informative,
    /// Warning item.
    Warning,
    /// Alert item.
    Alert,
}

/// How children of a menu item should be displayed.
/// Only one value is defined in the spec: "submenu".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildrenDisplay {
    /// Children should be displayed as a submenu.
    Submenu,
}

impl Default for ChildrenDisplay {
    fn default() -> Self {
        Self::Submenu
    }
}

impl From<&str> for ChildrenDisplay {
    fn from(s: &str) -> Self {
        match s {
            "submenu" => Self::Submenu,
            _ => Self::Submenu,
        }
    }
}

impl Display for ChildrenDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "submenu")
    }
}

impl Default for Disposition {
    fn default() -> Self {
        Self::Normal
    }
}

impl From<&str> for Disposition {
    fn from(s: &str) -> Self {
        match s {
            "informative" => Self::Informative,
            "warning" => Self::Warning,
            "alert" => Self::Alert,
            _ => Self::Normal,
        }
    }
}

impl Display for Disposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Informative => write!(f, "informative"),
            Self::Warning => write!(f, "warning"),
            Self::Alert => write!(f, "alert"),
        }
    }
}

/// DBusMenu event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuEvent {
    /// Item was clicked.
    Clicked,
    /// Mouse hovered over item.
    Hovered,
    /// Submenu was opened.
    Opened,
    /// Submenu was closed.
    Closed,
}

impl From<&str> for MenuEvent {
    fn from(s: &str) -> Self {
        match s {
            "clicked" => Self::Clicked,
            "hovered" => Self::Hovered,
            "opened" => Self::Opened,
            "closed" => Self::Closed,
            _ => Self::Clicked,
        }
    }
}

impl Display for MenuEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clicked => write!(f, "clicked"),
            Self::Hovered => write!(f, "hovered"),
            Self::Opened => write!(f, "opened"),
            Self::Closed => write!(f, "closed"),
        }
    }
}

/// Parsed menu item from DBusMenu.
///
/// Contains all official properties from the DBusMenu specification.
/// Properties map from com.canonical.dbusmenu as defined in libdbusmenu.
#[derive(Clone, PartialEq)]
pub struct MenuItem {
    /// Menu item ID (always present).
    pub id: i32,

    /// Menu item label text.
    ///
    /// default: empty string
    pub label: Option<String>,

    /// Whether the item is enabled (can be activated).
    ///
    /// default: true
    pub enabled: bool,

    /// Whether the item is visible.
    ///
    /// default: true
    pub visible: bool,

    /// Type of menu item.
    ///
    /// default: "standard"
    pub item_type: MenuItemType,

    /// Toggle type if applicable.
    ///
    /// default: none
    pub toggle_type: ToggleType,

    /// Toggle state if applicable.
    ///
    /// default: -1/unknown
    pub toggle_state: ToggleState,

    /// Icon name from the icon theme.
    pub icon_name: Option<String>,

    /// Raw icon data (typically PNG bytes).
    pub icon_data: Option<Vec<u8>>,

    /// Accessibility description for screen readers.
    pub accessible_desc: Option<String>,

    /// Keyboard shortcut arrays.
    ///
    /// array of arrays like [["Control", "q"]]
    pub shortcut: Option<Vec<Vec<String>>>,

    /// How to display this item.
    ///
    /// default: "normal"
    pub disposition: Disposition,

    /// How children should be displayed.
    ///
    /// Only one value exists in the spec: "submenu"
    pub children_display: ChildrenDisplay,

    /// Child menu items (may be empty).
    pub children: Vec<MenuItem>,
}

impl From<RawMenuLayout> for MenuItem {
    fn from(value: RawMenuLayout) -> Self {
        let root_menu = value.1;
        let root_menu_props = root_menu.1;
        let root_menu_children = root_menu.2;

        Self::from_props(root_menu.0, root_menu_props, root_menu_children)
    }
}

impl MenuItem {
    /// Check if this is a separator item.
    pub fn is_separator(&self) -> bool {
        self.item_type == MenuItemType::Separator
    }

    /// Check if this item has children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Check if this item has a submenu.
    pub fn has_submenu(&self) -> bool {
        !self.children.is_empty()
    }

    /// Check if this item is checkable.
    pub fn is_checkable(&self) -> bool {
        matches!(self.toggle_type, ToggleType::Checkmark | ToggleType::Radio)
    }

    fn from_props(
        id: i32,
        root_menu_props: HashMap<String, OwnedValue>,
        children: Vec<OwnedValue>,
    ) -> Self {
        let label = root_menu_props
            .get("label")
            .and_then(|v| String::try_from(v.clone()).ok());

        let enabled = root_menu_props
            .get("enabled")
            .and_then(|v| bool::try_from(v).ok())
            .unwrap_or(true);

        let visible = root_menu_props
            .get("visible")
            .and_then(|v| bool::try_from(v).ok())
            .unwrap_or(true);

        let item_type = root_menu_props
            .get("type")
            .and_then(|v| <&str>::try_from(v).ok())
            .map(MenuItemType::from)
            .unwrap_or_default();

        let toggle_type = root_menu_props
            .get("toggle-type")
            .and_then(|v| <&str>::try_from(v).ok())
            .map(ToggleType::from)
            .unwrap_or_default();

        let toggle_state = root_menu_props
            .get("toggle-state")
            .and_then(|v| <i32>::try_from(v).ok())
            .map(ToggleState::from)
            .unwrap_or_default();

        let icon_name = root_menu_props
            .get("icon-name")
            .and_then(|v| String::try_from(v.clone()).ok());

        let icon_data = root_menu_props
            .get("icon-data")
            .and_then(|v| Vec::<u8>::try_from(v.clone()).ok());

        let accessible_desc = root_menu_props
            .get("accessible-desc")
            .and_then(|v| String::try_from(v.clone()).ok());

        let shortcut = root_menu_props
            .get("shortcut")
            .and_then(|v| Vec::<Vec<String>>::try_from(v.clone()).ok());

        let disposition = root_menu_props
            .get("disposition")
            .and_then(|v| <&str>::try_from(v).ok())
            .map(Disposition::from)
            .unwrap_or_default();

        let children_display = root_menu_props
            .get("children-display")
            .and_then(|v| <&str>::try_from(v).ok())
            .map(ChildrenDisplay::from)
            .unwrap_or_default();

        let children = children
            .into_iter()
            .filter_map(|child| {
                let (child_id, child_props, child_children) =
                    <(i32, HashMap<String, OwnedValue>, Vec<OwnedValue>)>::try_from(child).ok()?;
                Some(Self::from_props(child_id, child_props, child_children))
            })
            .collect();

        Self {
            id,
            label,
            enabled,
            visible,
            item_type,
            toggle_type,
            toggle_state,
            icon_name,
            icon_data,
            accessible_desc,
            shortcut,
            disposition,
            children_display,
            children,
        }
    }
}

impl Debug for MenuItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("MenuItem");
        debug.field("id", &self.id);

        if let Some(label) = &self.label {
            debug.field("label", label);
        }

        if !self.enabled {
            debug.field("enabled", &self.enabled);
        }

        if !self.visible {
            debug.field("visible", &self.visible);
        }

        if self.item_type != MenuItemType::Standard {
            debug.field("item_type", &self.item_type);
        }

        if self.toggle_type != ToggleType::None {
            debug.field("toggle_type", &self.toggle_type);
            debug.field("toggle_state", &self.toggle_state);
        }

        if let Some(icon_name) = &self.icon_name {
            debug.field("icon_name", icon_name);
        }

        if let Some(icon_data) = &self.icon_data {
            debug.field("icon_data", &format!("<{} bytes>", icon_data.len()));
        }

        if let Some(desc) = &self.accessible_desc {
            debug.field("accessible_desc", desc);
        }

        if let Some(shortcut) = &self.shortcut {
            debug.field("shortcut", shortcut);
        }

        if self.disposition != Disposition::Normal {
            debug.field("disposition", &self.disposition);
        }

        if self.children_display != ChildrenDisplay::Submenu {
            debug.field("children_display", &self.children_display);
        }

        if !self.children.is_empty() {
            debug.field("children", &self.children);
        }

        debug.finish()
    }
}

/// Raw DBusMenu layout item.
#[derive(Debug, Clone)]
pub struct DBusMenuLayoutItem {
    /// Item ID.
    pub id: i32,
    /// Item properties.
    pub properties: HashMap<String, OwnedValue>,
    /// Child items.
    pub children: Vec<OwnedValue>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_item_type_from_str_with_separator_returns_separator() {
        let item_type = MenuItemType::from("separator");
        assert_eq!(item_type, MenuItemType::Separator);
    }

    #[test]
    fn menu_item_type_from_str_with_standard_returns_standard() {
        let item_type = MenuItemType::from("standard");
        assert_eq!(item_type, MenuItemType::Standard);
    }

    #[test]
    fn menu_item_type_from_str_with_unknown_returns_standard() {
        let item_type = MenuItemType::from("unknown");
        assert_eq!(item_type, MenuItemType::Standard);
    }

    #[test]
    fn toggle_type_from_str_with_checkmark_returns_checkmark() {
        let toggle_type = ToggleType::from("checkmark");
        assert_eq!(toggle_type, ToggleType::Checkmark);
    }

    #[test]
    fn toggle_type_from_str_with_radio_returns_radio() {
        let toggle_type = ToggleType::from("radio");
        assert_eq!(toggle_type, ToggleType::Radio);
    }

    #[test]
    fn toggle_type_from_str_with_unknown_returns_none() {
        let toggle_type = ToggleType::from("unknown");
        assert_eq!(toggle_type, ToggleType::None);
    }

    #[test]
    fn toggle_state_from_i32_with_zero_returns_unchecked() {
        let state = ToggleState::from(0);
        assert_eq!(state, ToggleState::Unchecked);
    }

    #[test]
    fn toggle_state_from_i32_with_one_returns_checked() {
        let state = ToggleState::from(1);
        assert_eq!(state, ToggleState::Checked);
    }

    #[test]
    fn toggle_state_from_i32_with_negative_returns_unknown() {
        let state = ToggleState::from(-1);
        assert_eq!(state, ToggleState::Unknown);
    }

    #[test]
    fn toggle_state_to_i32_unchecked_returns_zero() {
        let value: i32 = ToggleState::Unchecked.into();
        assert_eq!(value, 0);
    }

    #[test]
    fn toggle_state_to_i32_checked_returns_one() {
        let value: i32 = ToggleState::Checked.into();
        assert_eq!(value, 1);
    }

    #[test]
    fn toggle_state_to_i32_unknown_returns_negative_one() {
        let value: i32 = ToggleState::Unknown.into();
        assert_eq!(value, -1);
    }

    #[test]
    fn disposition_from_str_with_informative_returns_informative() {
        let disposition = Disposition::from("informative");
        assert_eq!(disposition, Disposition::Informative);
    }

    #[test]
    fn disposition_from_str_with_warning_returns_warning() {
        let disposition = Disposition::from("warning");
        assert_eq!(disposition, Disposition::Warning);
    }

    #[test]
    fn disposition_from_str_with_alert_returns_alert() {
        let disposition = Disposition::from("alert");
        assert_eq!(disposition, Disposition::Alert);
    }

    #[test]
    fn disposition_from_str_with_unknown_returns_normal() {
        let disposition = Disposition::from("unknown");
        assert_eq!(disposition, Disposition::Normal);
    }

    #[test]
    fn children_display_from_str_always_returns_submenu() {
        assert_eq!(ChildrenDisplay::from("submenu"), ChildrenDisplay::Submenu);
        assert_eq!(ChildrenDisplay::from("unknown"), ChildrenDisplay::Submenu);
        assert_eq!(ChildrenDisplay::from(""), ChildrenDisplay::Submenu);
    }

    #[test]
    fn menu_event_from_str_with_clicked_returns_clicked() {
        let event = MenuEvent::from("clicked");
        assert_eq!(event, MenuEvent::Clicked);
    }

    #[test]
    fn menu_event_from_str_with_hovered_returns_hovered() {
        let event = MenuEvent::from("hovered");
        assert_eq!(event, MenuEvent::Hovered);
    }

    #[test]
    fn menu_event_from_str_with_opened_returns_opened() {
        let event = MenuEvent::from("opened");
        assert_eq!(event, MenuEvent::Opened);
    }

    #[test]
    fn menu_event_from_str_with_closed_returns_closed() {
        let event = MenuEvent::from("closed");
        assert_eq!(event, MenuEvent::Closed);
    }

    #[test]
    fn menu_event_from_str_with_unknown_returns_clicked() {
        let event = MenuEvent::from("unknown");
        assert_eq!(event, MenuEvent::Clicked);
    }

    #[test]
    fn menu_item_is_separator_with_separator_type_returns_true() {
        let item = MenuItem {
            id: 1,
            label: None,
            enabled: true,
            visible: true,
            item_type: MenuItemType::Separator,
            toggle_type: ToggleType::None,
            toggle_state: ToggleState::Unchecked,
            icon_name: None,
            icon_data: None,
            accessible_desc: None,
            shortcut: None,
            disposition: Disposition::Normal,
            children_display: ChildrenDisplay::Submenu,
            children: vec![],
        };

        assert!(item.is_separator());
    }

    #[test]
    fn menu_item_is_separator_with_standard_type_returns_false() {
        let item = MenuItem {
            id: 1,
            label: None,
            enabled: true,
            visible: true,
            item_type: MenuItemType::Standard,
            toggle_type: ToggleType::None,
            toggle_state: ToggleState::Unchecked,
            icon_name: None,
            icon_data: None,
            accessible_desc: None,
            shortcut: None,
            disposition: Disposition::Normal,
            children_display: ChildrenDisplay::Submenu,
            children: vec![],
        };

        assert!(!item.is_separator());
    }

    #[test]
    fn menu_item_has_children_with_empty_vec_returns_false() {
        let item = MenuItem {
            id: 1,
            label: None,
            enabled: true,
            visible: true,
            item_type: MenuItemType::Standard,
            toggle_type: ToggleType::None,
            toggle_state: ToggleState::Unchecked,
            icon_name: None,
            icon_data: None,
            accessible_desc: None,
            shortcut: None,
            disposition: Disposition::Normal,
            children_display: ChildrenDisplay::Submenu,
            children: vec![],
        };

        assert!(!item.has_children());
    }

    #[test]
    fn menu_item_has_children_with_items_returns_true() {
        let child = MenuItem {
            id: 2,
            label: Some(String::from("Child")),
            enabled: true,
            visible: true,
            item_type: MenuItemType::Standard,
            toggle_type: ToggleType::None,
            toggle_state: ToggleState::Unchecked,
            icon_name: None,
            icon_data: None,
            accessible_desc: None,
            shortcut: None,
            disposition: Disposition::Normal,
            children_display: ChildrenDisplay::Submenu,
            children: vec![],
        };

        let item = MenuItem {
            id: 1,
            label: None,
            enabled: true,
            visible: true,
            item_type: MenuItemType::Standard,
            toggle_type: ToggleType::None,
            toggle_state: ToggleState::Unchecked,
            icon_name: None,
            icon_data: None,
            accessible_desc: None,
            shortcut: None,
            disposition: Disposition::Normal,
            children_display: ChildrenDisplay::Submenu,
            children: vec![child],
        };

        assert!(item.has_children());
    }

    #[test]
    fn menu_item_has_submenu_with_empty_vec_returns_false() {
        let item = MenuItem {
            id: 1,
            label: None,
            enabled: true,
            visible: true,
            item_type: MenuItemType::Standard,
            toggle_type: ToggleType::None,
            toggle_state: ToggleState::Unchecked,
            icon_name: None,
            icon_data: None,
            accessible_desc: None,
            shortcut: None,
            disposition: Disposition::Normal,
            children_display: ChildrenDisplay::Submenu,
            children: vec![],
        };

        assert!(!item.has_submenu());
    }

    #[test]
    fn menu_item_has_submenu_with_items_returns_true() {
        let child = MenuItem {
            id: 2,
            label: Some(String::from("Child")),
            enabled: true,
            visible: true,
            item_type: MenuItemType::Standard,
            toggle_type: ToggleType::None,
            toggle_state: ToggleState::Unchecked,
            icon_name: None,
            icon_data: None,
            accessible_desc: None,
            shortcut: None,
            disposition: Disposition::Normal,
            children_display: ChildrenDisplay::Submenu,
            children: vec![],
        };

        let item = MenuItem {
            id: 1,
            label: None,
            enabled: true,
            visible: true,
            item_type: MenuItemType::Standard,
            toggle_type: ToggleType::None,
            toggle_state: ToggleState::Unchecked,
            icon_name: None,
            icon_data: None,
            accessible_desc: None,
            shortcut: None,
            disposition: Disposition::Normal,
            children_display: ChildrenDisplay::Submenu,
            children: vec![child],
        };

        assert!(item.has_submenu());
    }

    #[test]
    fn menu_item_is_checkable_with_checkmark_returns_true() {
        let item = MenuItem {
            id: 1,
            label: None,
            enabled: true,
            visible: true,
            item_type: MenuItemType::Standard,
            toggle_type: ToggleType::Checkmark,
            toggle_state: ToggleState::Unchecked,
            icon_name: None,
            icon_data: None,
            accessible_desc: None,
            shortcut: None,
            disposition: Disposition::Normal,
            children_display: ChildrenDisplay::Submenu,
            children: vec![],
        };

        assert!(item.is_checkable());
    }

    #[test]
    fn menu_item_is_checkable_with_radio_returns_true() {
        let item = MenuItem {
            id: 1,
            label: None,
            enabled: true,
            visible: true,
            item_type: MenuItemType::Standard,
            toggle_type: ToggleType::Radio,
            toggle_state: ToggleState::Unchecked,
            icon_name: None,
            icon_data: None,
            accessible_desc: None,
            shortcut: None,
            disposition: Disposition::Normal,
            children_display: ChildrenDisplay::Submenu,
            children: vec![],
        };

        assert!(item.is_checkable());
    }

    #[test]
    fn menu_item_is_checkable_with_none_returns_false() {
        let item = MenuItem {
            id: 1,
            label: None,
            enabled: true,
            visible: true,
            item_type: MenuItemType::Standard,
            toggle_type: ToggleType::None,
            toggle_state: ToggleState::Unchecked,
            icon_name: None,
            icon_data: None,
            accessible_desc: None,
            shortcut: None,
            disposition: Disposition::Normal,
            children_display: ChildrenDisplay::Submenu,
            children: vec![],
        };

        assert!(!item.is_checkable());
    }
}
