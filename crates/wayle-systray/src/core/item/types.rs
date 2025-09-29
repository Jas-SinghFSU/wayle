use zbus::zvariant::OwnedObjectPath;

use crate::types::{
    item::{Category, IconPixmap, Status, Tooltip},
    menu::RawMenuLayout,
};

pub(crate) struct TrayItemProperties {
    pub id: String,
    pub title: String,
    pub category: Category,
    pub status: Status,
    pub window_id: u32,
    pub item_is_menu: bool,
    pub icon_name: Option<String>,
    pub icon_pixmap: Vec<IconPixmap>,
    pub overlay_icon_name: Option<String>,
    pub overlay_icon_pixmap: Vec<IconPixmap>,
    pub attention_icon_name: Option<String>,
    pub attention_icon_pixmap: Vec<IconPixmap>,
    pub attention_movie_name: Option<String>,
    pub icon_theme_path: Option<String>,
    pub tooltip: Tooltip,
    pub menu_path: OwnedObjectPath,
    pub menu: Option<RawMenuLayout>,
}
