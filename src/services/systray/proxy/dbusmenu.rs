use zbus::{Result, proxy, zvariant::OwnedValue};

use crate::services::systray::types::menu::{
    RawMenuItemKeysList, RawMenuItemsPropsList, RawMenuLayout,
};

/// DBusMenu protocol proxy for menu structures over DBus.
///
/// Based on the com.canonical.dbusmenu specification from libdbusmenu.
/// Source: https://github.com/JetBrains/libdbusmenu
///
/// Service: `[same as StatusNotifierItem service]`
/// Interface: `com.canonical.dbusmenu`
/// Object path: `[from StatusNotifierItem's Menu property]`
#[proxy(interface = "com.canonical.dbusmenu")]
pub(crate) trait DBusMenu {
    /// This is called by the applet to notify the application that it is about
    /// to show the menu under the specified item.
    ///
    /// # Arguments
    /// * `id` - Which menu item represents the parent of the item about to be shown
    ///
    /// # Returns
    /// * `needUpdate` - Whether this AboutToShow event should result in the menu being updated
    fn about_to_show(&self, id: i32) -> Result<bool>;

    /// A function to tell several menus being shown that they are about to
    /// be shown to the user. This is likely only useful for programmatic purposes
    /// so while the return values are returned, in general, the singular function
    /// should be used in most user interaction scenarios.
    ///
    /// # Arguments
    /// * `ids` - The IDs of the menu items whose submenus are being shown
    ///
    /// # Returns
    /// * `updatesNeeded` - The IDs of the menus that need updates
    /// * `idErrors` - List of menuitem IDs that couldn't be found
    fn about_to_show_group(&self, ids: Vec<i32>) -> Result<(Vec<i32>, Vec<i32>)>;

    /// This is called by the applet to notify the application an event happened on a
    /// menu item.
    ///
    /// Event types can be:
    /// - "clicked"
    /// - "hovered"
    /// - "opened"
    /// - "closed"
    ///
    /// Vendor specific events can be added by prefixing them with "x-<vendor>-"
    ///
    /// # Arguments
    /// * `id` - The id of the item which received the event
    /// * `event_id` - The type of event
    /// * `data` - Event-specific data
    /// * `timestamp` - The time that the event occurred if available or the time the message was sent if not
    #[zbus(no_reply)]
    fn event(&self, id: i32, event_id: &str, data: OwnedValue, timestamp: u32) -> Result<()>;

    /// Used to pass a set of events as a single message for possibly several
    /// different menuitems. This is done to optimize DBus traffic.
    ///
    /// # Arguments
    /// * `events` - An array of all the events that should be passed. Each tuple contains:
    ///              (id, eventID, data, timestamp)
    ///
    /// # Returns
    /// * `idErrors` - List of menuitem IDs that couldn't be found
    fn event_group(&self, events: Vec<(i32, String, OwnedValue, u32)>) -> Result<Vec<i32>>;

    /// Get a single property on a single item. This is not useful if you're
    /// going to implement this interface, it should only be used if you're
    /// debugging via a commandline tool.
    ///
    /// # Arguments
    /// * `id` - The id of the item to get the property from
    /// * `property` - The name of the property to get
    ///
    /// # Returns
    /// * The value of the property
    fn get_property(&self, id: i32, property: &str) -> Result<OwnedValue>;

    /// Provides the layout and properties that are attached to the entries
    /// that are in the layout. It only gives the items that are children
    /// of the item that is specified in parentId. It will return all of the
    /// properties or specific ones depending on the value in propertyNames.
    ///
    /// The format is recursive, where the second 'v' is in the same format
    /// as the original 'a(ia{sv}av)'. Its contents depends on the value
    /// of recursionDepth.
    ///
    /// # Arguments
    /// * `parent_id` - The ID of the parent node for the layout. For
    ///                 grabbing the layout from the root node use zero.
    /// * `recursion_depth` - The amount of levels of recursion to use when building
    ///                       the layout. This affects the content of the second
    ///                       variant array in the returned layout. The value of -1
    ///                       indicates to recurse all of the items. Zero means that
    ///                       no recursion should be done and only one level of children
    ///                       should be retrieved.
    /// * `property_names` - The list of item properties we are interested in. If there
    ///                      are no entries in the list all of the properties will be sent.
    ///
    /// # Returns
    /// * `(revision, layout)` - The layout as a recursive structure
    fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        property_names: Vec<String>,
    ) -> Result<RawMenuLayout>;

    /// Returns the list of items which are children of parentId.
    ///
    /// # Arguments
    /// * `ids` - A list of ids that we should be finding the properties
    ///           on. If the list is empty, all menu items should be sent.
    /// * `property_names` - The list of item properties we are
    ///                      interested in. If there are no entries in the list all of
    ///                      the properties will be sent.
    ///
    /// # Returns
    /// * An array of property values where each item is a struct of
    ///   (id, properties) where properties is a map of string to variant
    fn get_group_properties(
        &self,
        ids: Vec<i32>,
        property_names: Vec<String>,
    ) -> Result<RawMenuItemsPropsList>;

    /// Provides the version of the DBusMenu protocol that this API is
    /// implementing.
    #[zbus(property)]
    fn version(&self) -> Result<u32>;

    /// Tells if the menus are in a normal state or they believe that they
    /// could use some attention. Cases for showing them would be if help
    /// were referring to them or they accessors were being highlighted.
    /// This property can have two values: "normal" or "notice"
    #[zbus(property)]
    fn status(&self) -> Result<String>;

    /// The text direction of the current menuitem. This value should be
    /// propagated to all children of this menuitem.
    #[zbus(property)]
    fn text_direction(&self) -> Result<String>;

    /// A list of directories that should be used for finding icons using
    /// the icon naming spec. Ideally there should be an icon in the icon
    /// theme paths that match.
    #[zbus(property)]
    fn icon_theme_path(&self) -> Result<Vec<String>>;

    /// Triggered when there are lots of property updates across many items
    /// so they all get grouped into a single dbus message. The format is
    /// the ID of the item with a hashtable of names and values for those
    /// properties.
    #[zbus(signal)]
    fn items_properties_updated(
        &self,
        updated_props: RawMenuItemsPropsList,
        removed_props: RawMenuItemKeysList,
    ) -> Result<()>;

    /// The layout has changed in some way.
    ///
    /// # Arguments
    /// * `revision` - The revision number of the layout. For matching with GetLayout.
    /// * `parent` - If the layout change only affects a subtree of the
    ///              layout, this is the parent id of the subtree. If the
    ///              layout change affects the whole layout, parent is 0.
    #[zbus(signal)]
    fn layout_updated(&self, revision: u32, parent: i32) -> Result<()>;

    /// The server is requesting that all clients displaying this
    /// menu open it to the user. This would be for things like
    /// hotkeys that when the user presses them the menu should
    /// open and display itself to the user.
    ///
    /// # Arguments
    /// * `id` - ID of the menu that should be activated
    /// * `timestamp` - The time that the event occurred
    #[zbus(signal)]
    fn item_activation_requested(&self, id: i32, timestamp: u32) -> Result<()>;
}
