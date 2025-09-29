/// StatusNotifierItem type definitions.
pub mod item;
/// DBusMenu type definitions.
pub mod menu;

use item::ScrollOrientation;
use serde::{Deserialize, Serialize};

/// Coordinates for mouse events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coordinates {
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
}

impl Coordinates {
    /// Creates new coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Scroll delta and orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScrollDelta {
    /// Number of scroll steps (positive = down/right, negative = up/left).
    pub delta: i32,
    /// Whether scrolling is vertical or horizontal.
    pub orientation: ScrollOrientation,
}

/// Protocol version for StatusNotifierWatcher.
pub const PROTOCOL_VERSION: i32 = 0;

/// Well-known bus name for StatusNotifierWatcher.
pub const WATCHER_BUS_NAME: &str = "org.kde.StatusNotifierWatcher";

/// Object path for StatusNotifierWatcher.
pub const WATCHER_OBJECT_PATH: &str = "/StatusNotifierWatcher";

/// Interface name for StatusNotifierWatcher.
pub const WATCHER_INTERFACE: &str = "org.kde.StatusNotifierWatcher";

/// Object path for StatusNotifierItem.
pub const ITEM_OBJECT_PATH: &str = "/StatusNotifierItem";

/// Interface name for StatusNotifierItem.
pub const ITEM_INTERFACE: &str = "org.kde.StatusNotifierItem";

/// Operating mode for the system tray service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayMode {
    /// Act as the StatusNotifierWatcher (registry for items).
    Watcher,
    /// Act as a StatusNotifierHost (consumer of items).
    Host,
    /// Auto-detect based on whether watcher name is available.
    Auto,
}
