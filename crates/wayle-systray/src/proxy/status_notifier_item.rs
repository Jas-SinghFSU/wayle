use zbus::{Result, proxy, zvariant::OwnedObjectPath};

use crate::types::item::{RawPixmaps, RawTooltip};

#[proxy(
    interface = "org.kde.StatusNotifierItem",
    default_path = "/StatusNotifierItem"
)]
pub(crate) trait StatusNotifierItem {
    fn context_menu(&self, x: i32, y: i32) -> Result<()>;

    fn activate(&self, x: i32, y: i32) -> Result<()>;

    fn secondary_activate(&self, x: i32, y: i32) -> Result<()>;

    fn scroll(&self, delta: i32, orientation: &str) -> Result<()>;

    #[zbus(property)]
    fn category(&self) -> Result<String>;

    #[zbus(property)]
    fn id(&self) -> Result<String>;

    #[zbus(property)]
    fn title(&self) -> Result<String>;

    #[zbus(property)]
    fn status(&self) -> Result<String>;

    #[zbus(property)]
    fn window_id(&self) -> Result<u32>;

    #[zbus(property)]
    fn icon_name(&self) -> Result<String>;

    #[zbus(property)]
    fn icon_pixmap(&self) -> Result<RawPixmaps>;

    #[zbus(property)]
    fn overlay_icon_name(&self) -> Result<String>;

    #[zbus(property)]
    fn overlay_icon_pixmap(&self) -> Result<RawPixmaps>;

    #[zbus(property)]
    fn attention_icon_name(&self) -> Result<String>;

    #[zbus(property)]
    fn attention_icon_pixmap(&self) -> Result<RawPixmaps>;

    #[zbus(property)]
    fn attention_movie_name(&self) -> Result<String>;

    #[zbus(property)]
    fn tool_tip(&self) -> Result<RawTooltip>;

    #[zbus(property)]
    fn item_is_menu(&self) -> Result<bool>;

    #[zbus(property)]
    fn menu(&self) -> Result<OwnedObjectPath>;

    #[zbus(property)]
    fn icon_theme_path(&self) -> Result<String>;

    #[zbus(signal)]
    fn new_title(&self) -> Result<()>;

    #[zbus(signal)]
    fn new_icon(&self) -> Result<()>;

    #[zbus(signal)]
    fn new_attention_icon(&self) -> Result<()>;

    #[zbus(signal)]
    fn new_overlay_icon(&self) -> Result<()>;

    #[zbus(signal)]
    fn new_tool_tip(&self) -> Result<()>;

    #[zbus(signal)]
    fn new_status(&self, status: String) -> Result<()>;
}
