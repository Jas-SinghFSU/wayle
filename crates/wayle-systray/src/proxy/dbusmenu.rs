use zbus::{Result, proxy, zvariant::OwnedValue};

use crate::types::menu::{RawMenuItemKeysList, RawMenuItemsPropsList, RawMenuLayout};

#[proxy(interface = "com.canonical.dbusmenu")]
pub(crate) trait DBusMenu {
    fn about_to_show(&self, id: i32) -> Result<bool>;

    fn about_to_show_group(&self, ids: Vec<i32>) -> Result<(Vec<i32>, Vec<i32>)>;

    #[zbus(no_reply)]
    fn event(&self, id: i32, event_id: &str, data: OwnedValue, timestamp: u32) -> Result<()>;

    fn event_group(&self, events: Vec<(i32, String, OwnedValue, u32)>) -> Result<Vec<i32>>;

    fn get_property(&self, id: i32, property: &str) -> Result<OwnedValue>;

    fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        property_names: Vec<String>,
    ) -> Result<RawMenuLayout>;

    fn get_group_properties(
        &self,
        ids: Vec<i32>,
        property_names: Vec<String>,
    ) -> Result<RawMenuItemsPropsList>;

    #[zbus(property)]
    fn version(&self) -> Result<u32>;

    #[zbus(property)]
    fn status(&self) -> Result<String>;

    #[zbus(property)]
    fn text_direction(&self) -> Result<String>;

    #[zbus(property)]
    fn icon_theme_path(&self) -> Result<Vec<String>>;

    #[zbus(signal)]
    fn items_properties_updated(
        &self,
        updated_props: RawMenuItemsPropsList,
        removed_props: RawMenuItemKeysList,
    ) -> Result<()>;

    #[zbus(signal)]
    fn layout_updated(&self, revision: u32, parent: i32) -> Result<()>;

    #[zbus(signal)]
    fn item_activation_requested(&self, id: i32, timestamp: u32) -> Result<()>;
}
