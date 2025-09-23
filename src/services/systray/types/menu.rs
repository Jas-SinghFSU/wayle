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

impl MenuItem {
    /// Create a new menu item with default values.
    pub fn new(id: i32) -> Self {
        Self {
            id,
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
            children: Vec::new(),
        }
    }

    /// Create a new menu item with a label.
    pub fn with_label(id: i32, label: impl Into<String>) -> Self {
        let mut item = Self::new(id);
        item.label = Some(label.into());
        item
    }

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
