use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

use serde::{Deserialize, Serialize};
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MenuItemType {
    /// Standard clickable menu item.
    #[serde(rename = "standard")]
    Standard,
    /// Menu separator.
    #[serde(rename = "separator")]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Standard => write!(f, "standard"),
            Self::Separator => write!(f, "separator"),
        }
    }
}

/// Toggle type for checkable menu items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToggleType {
    /// No toggle capability.
    #[serde(rename = "none")]
    None,
    /// Checkbox (independent toggle).
    #[serde(rename = "checkmark")]
    Checkmark,
    /// Radio button (mutually exclusive within group).
    #[serde(rename = "radio")]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::None => write!(f, ""),
            Self::Checkmark => write!(f, "checkmark"),
            Self::Radio => write!(f, "radio"),
        }
    }
}

/// Toggle state for checkable menu items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToggleState {
    /// Unchecked state.
    #[serde(rename = "unchecked")]
    Unchecked,
    /// Checked state.
    #[serde(rename = "checked")]
    Checked,
    /// Indeterminate state.
    #[serde(rename = "unknown")]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Disposition {
    /// Normal menu item.
    #[serde(rename = "normal")]
    Normal,
    /// Informative item.
    #[serde(rename = "informative")]
    Informative,
    /// Warning item.
    #[serde(rename = "warning")]
    Warning,
    /// Alert item.
    #[serde(rename = "alert")]
    Alert,
}

/// How children of a menu item should be displayed.
/// Only one value is defined in the spec: "submenu".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildrenDisplay {
    /// Children should be displayed as a submenu.
    #[serde(rename = "submenu")]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Informative => write!(f, "informative"),
            Self::Warning => write!(f, "warning"),
            Self::Alert => write!(f, "alert"),
        }
    }
}

/// DBusMenu event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MenuEvent {
    /// Item was clicked.
    #[serde(rename = "clicked")]
    Clicked,
    /// Mouse hovered over item.
    #[serde(rename = "hovered")]
    Hovered,
    /// Submenu was opened.
    #[serde(rename = "opened")]
    Opened,
    /// Submenu was closed.
    #[serde(rename = "closed")]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
