mod controls;
mod monitoring;
mod types;

use std::sync::Arc;

use controls::TrayItemController;
use futures::{Stream, StreamExt};
use tokio_util::sync::CancellationToken;
use tracing::instrument;
use types::TrayItemProperties;
use wayle_common::{Property, unwrap_bool, unwrap_string, unwrap_u32};
use wayle_traits::{ModelMonitoring, Reactive};
use zbus::{
    Connection,
    proxy::CacheProperties,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::{
    error::Error,
    proxy::{dbusmenu::DBusMenuProxy, status_notifier_item::StatusNotifierItemProxy},
    types::{
        Coordinates,
        item::{Category, IconPixmap, Status, Tooltip},
        menu::{MenuEvent, MenuItem, RawMenuItemsPropsList},
    },
};

/// Parsed D-Bus service identifier with separate service name and object path.
struct ServiceIdentifier<'a> {
    /// The D-Bus service/bus name (e.g., ":1.234" or "org.kde.StatusNotifierItem-4077-1").
    service: &'a str,
    /// The object path (e.g., "/StatusNotifierItem" or "/some/nested/path").
    path: &'a str,
}

/// StatusNotifierItem representation with associated DBusMenu.
///
/// Combines the org.kde.StatusNotifierItem and com.canonical.dbusmenu
/// interfaces into a single model for system tray items.
#[derive(Debug, Clone)]
pub struct TrayItem {
    pub(crate) zbus_connection: Connection,
    pub(crate) cancellation_token: Option<CancellationToken>,

    /// D-Bus service name or path (e.g., "org.kde.StatusNotifierItem-12345-1")
    pub bus_name: Property<String>,

    /// It's a name that should be unique for this application and consistent between sessions,
    /// such as the application name itself.
    pub id: Property<String>,

    /// It's a name that describes the application, it can be more descriptive than Id.
    pub title: Property<String>,

    /// Describes the category of this item.
    pub category: Property<Category>,

    /// Describes the status of this item or of the associated application.
    pub status: Property<Status>,

    /// It's the windowing-system dependent identifier for a window, the application can chose one
    /// of its windows to be available trough this property or just set 0 if it's not interested.
    pub window_id: Property<u32>,

    /// The item only support the context menu, the visualization should prefer showing the menu
    /// or sending `ContextMenu()` instead of `Activate()`
    pub item_is_menu: Property<bool>,

    /// The StatusNotifierItem can carry an icon that can be used by the visualization to identify
    /// the item. An icon can either be identified by its Freedesktop-compliant icon name, carried
    /// by this property of by the icon data itself, carried by the property IconPixmap.
    pub icon_name: Property<Option<String>>,

    /// Carries an ARGB32 binary representation of the icon.
    pub icon_pixmap: Property<Vec<IconPixmap>>,

    /// The Freedesktop-compliant name of an icon. This can be used by the visualization to
    /// indicate extra state information, for instance as an overlay for the main icon.
    pub overlay_icon_name: Property<Option<String>>,

    /// ARGB32 binary representation of the overlay icon.
    pub overlay_icon_pixmap: Property<Vec<IconPixmap>>,

    /// The Freedesktop-compliant name of an icon. this can be used by the visualization to
    /// indicate that the item is in RequestingAttention state.
    pub attention_icon_name: Property<Option<String>>,

    /// ARGB32 binary representation of the requesting attention icon.
    pub attention_icon_pixmap: Property<Vec<IconPixmap>>,

    /// An item can also specify an animation associated to the RequestingAttention state.
    /// This should be either a Freedesktop-compliant icon name or a full path.
    pub attention_movie_name: Property<Option<String>>,

    /// An additional path to add to the theme search path to find the icons specified above.
    pub icon_theme_path: Property<Option<String>>,

    /// Data structure that contains information for a tooltip.
    pub tooltip: Property<Tooltip>,

    /// Hierarchical menu structure from DBusMenu interface.
    pub menu: Property<Option<MenuItem>>,

    /// DBus path to an object which should implement the com.canonical.dbusmenu interface.
    pub menu_path: Property<OwnedObjectPath>,
}

impl PartialEq for TrayItem {
    fn eq(&self, other: &Self) -> bool {
        self.bus_name.get() == other.bus_name.get()
    }
}

/// Parameters for creating a TrayItem instance.
///
/// **Note**: This type is exposed for trait implementation requirements
/// but should not be constructed directly by external consumers.
#[doc(hidden)]
pub struct TrayItemParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) service: String,
}

/// Parameters for creating a live TrayItem instance.
///
/// **Note**: This type is exposed for trait implementation requirements
/// but should not be constructed directly by external consumers.
#[doc(hidden)]
pub struct LiveTrayItemParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) service: String,
    pub(crate) cancellation_token: &'a CancellationToken,
}

impl Reactive for TrayItem {
    type Error = Error;
    type Context<'a> = TrayItemParams<'a>;
    type LiveContext<'a> = LiveTrayItemParams<'a>;

    #[instrument(skip(context), fields(service = %context.service), err)]
    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error> {
        let props = Self::fetch_properties(context.connection, &context.service).await?;
        Ok(Self::from_properties(
            props,
            context.connection.clone(),
            context.service.clone(),
            None,
        ))
    }

    #[instrument(skip(context), fields(service = %context.service), err)]
    async fn get_live(context: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let props = Self::fetch_properties(context.connection, &context.service).await?;
        let item = Self::from_properties(
            props,
            context.connection.clone(),
            context.service.clone(),
            Some(context.cancellation_token.child_token()),
        );

        let item = Arc::new(item);

        item.clone().start_monitoring().await?;

        Ok(item)
    }
}

impl TrayItem {
    /// Asks the status notifier item to show a context menu, this is typically a consequence of
    /// user input, such as mouse right click over the graphical representation of the item.
    ///
    /// The x and y parameters are in screen coordinates and is to be considered an hint to the
    /// item about where to show the context menu.
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus call fails or the item is unreachable.
    #[instrument(
        skip(self),
        fields(bus_name = %self.bus_name.get(), x = coords.x, y = coords.y),
        err
    )]
    pub async fn context_menu(&self, coords: Coordinates) -> Result<(), Error> {
        let bus_name = self.bus_name.get();
        let id = Self::parse_service_identifier(&bus_name);
        TrayItemController::context_menu(
            &self.zbus_connection,
            id.service,
            id.path,
            coords.x,
            coords.y,
        )
        .await
    }

    /// Asks the status notifier item for activation, this is typically a consequence of user
    /// input, such as mouse left click over the graphical representation of the item. The
    /// application will perform any task is considered appropriate as an activation request.
    ///
    /// The `x` and `y` parameters are in screen coordinates and is to be considered an hint to the
    /// item where to show eventual windows (if any).
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus call fails or the item is unreachable.
    #[instrument(
        skip(self),
        fields(bus_name = %self.bus_name.get(), x = coords.x, y = coords.y),
        err
    )]
    pub async fn activate(&self, coords: Coordinates) -> Result<(), Error> {
        let bus_name = self.bus_name.get();
        let id = Self::parse_service_identifier(&bus_name);
        TrayItemController::activate(
            &self.zbus_connection,
            id.service,
            id.path,
            coords.x,
            coords.y,
        )
        .await
    }

    /// Is to be considered a secondary and less important form of activation compared to
    /// Activate. This is typically a consequence of user input, such as mouse middle click over
    /// the graphical representation of the item. The application will perform any task is
    /// considered appropriate as an activation request.
    ///
    /// The `x` and `y` parameters are in screen coordinates and is to be considered an hint to the
    /// item where to show eventual windows (if any).
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus call fails or the item is unreachable.
    #[instrument(
        skip(self),
        fields(bus_name = %self.bus_name.get(), x = coords.x, y = coords.y),
        err
    )]
    pub async fn secondary_activate(&self, coords: Coordinates) -> Result<(), Error> {
        let bus_name = self.bus_name.get();
        let id = Self::parse_service_identifier(&bus_name);
        TrayItemController::secondary_activate(
            &self.zbus_connection,
            id.service,
            id.path,
            coords.x,
            coords.y,
        )
        .await
    }

    /// The user asked for a scroll action. This is caused from input such as mouse wheel over
    /// the graphical representation of the item.
    ///
    /// The `orientation` parameter can be either horizontal or vertical.
    /// The amount of scroll is represented by `delta`: a positive value represents a scroll down
    /// or right, a negative value represents a scroll up or left.
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus call fails or the item is unreachable.
    #[instrument(
        skip(self),
        fields(bus_name = %self.bus_name.get(), delta, orientation = %orientation),
        err
    )]
    pub async fn scroll(&self, delta: i32, orientation: &str) -> Result<(), Error> {
        let bus_name = self.bus_name.get();
        let id = Self::parse_service_identifier(&bus_name);
        TrayItemController::scroll(
            &self.zbus_connection,
            id.service,
            id.path,
            delta,
            orientation,
        )
        .await
    }

    /// Refreshes the root menu by calling AboutToShow on the menu root.
    ///
    /// This should be called before displaying a menu to ensure applications
    /// can populate dynamic content. Part of the DBusMenu protocol for lazy menu population.
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus call fails or the menu is unreachable.
    #[instrument(skip(self), fields(bus_name = %self.bus_name.get()), err)]
    pub async fn refresh_menu(&self) -> Result<bool, Error> {
        const MENU_ID: i32 = 0;
        let bus_name = self.bus_name.get();
        let service_id = Self::parse_service_identifier(&bus_name);
        TrayItemController::menu_about_to_show(
            &self.zbus_connection,
            service_id.service,
            self.menu_path.get().as_str(),
            MENU_ID,
        )
        .await
    }

    /// Notifies the application that a menu or submenu is about to be shown.
    ///
    /// This allows applications to populate menus on-demand rather than keeping
    /// all menu content in memory. Called automatically by menu rendering adapters.
    ///
    /// # Arguments
    /// * `id` - Menu item ID representing the parent of the item about to be shown (0 for root)
    ///
    /// # Returns
    /// * `true` if the menu needs to be updated, `false` otherwise
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus call fails or the menu is unreachable.
    #[instrument(skip(self), fields(bus_name = %self.bus_name.get(), id), err)]
    pub async fn menu_about_to_show(&self, id: i32) -> Result<bool, Error> {
        let bus_name = self.bus_name.get();
        let service_id = Self::parse_service_identifier(&bus_name);
        TrayItemController::menu_about_to_show(
            &self.zbus_connection,
            service_id.service,
            self.menu_path.get().as_str(),
            id,
        )
        .await
    }

    /// Sends a menu event to the application.
    ///
    /// # Arguments
    /// * `id` - Menu item ID that received the event
    /// * `event` - Type of event
    /// * `timestamp` - Unix timestamp in seconds
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus call fails or the menu is unreachable.
    #[instrument(
        skip(self),
        fields(bus_name = %self.bus_name.get(), id, event = ?event, timestamp),
        err
    )]
    pub async fn menu_event(&self, id: i32, event: MenuEvent, timestamp: u32) -> Result<(), Error> {
        let bus_name = self.bus_name.get();
        let service_id = Self::parse_service_identifier(&bus_name);
        TrayItemController::menu_event(
            &self.zbus_connection,
            service_id.service,
            self.menu_path.get().as_str(),
            id,
            &event.to_string(),
            OwnedValue::from(0i32),
            timestamp,
        )
        .await
    }

    /// Notifies the application that multiple menus are about to be shown.
    ///
    /// Batch version of `menu_about_to_show` for programmatic use.
    ///
    /// # Arguments
    /// * `ids` - Menu item IDs whose submenus are being shown
    ///
    /// # Returns
    /// * `(updates_needed, id_errors)` - IDs needing updates and IDs not found
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus call fails or the menu is unreachable.
    pub async fn menu_about_to_show_group(
        &self,
        ids: Vec<i32>,
    ) -> Result<(Vec<i32>, Vec<i32>), Error> {
        TrayItemController::menu_about_to_show_group(
            &self.zbus_connection,
            &self.bus_name.get(),
            self.menu_path.get().as_str(),
            ids,
        )
        .await
    }

    /// Sends multiple menu events to the application.
    ///
    /// Batch version of `menu_event` to optimize D-Bus traffic.
    ///
    /// # Arguments
    /// * `events` - Array of (id, event, timestamp) tuples
    ///
    /// # Returns
    /// * List of menu item IDs that couldn't be found
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus call fails or the menu is unreachable.
    pub async fn menu_event_group(
        &self,
        events: Vec<(i32, MenuEvent, u32)>,
    ) -> Result<Vec<i32>, Error> {
        let dbus_events = events
            .into_iter()
            .map(|(id, event, timestamp)| {
                (id, event.to_string(), OwnedValue::from(0i32), timestamp)
            })
            .collect();

        TrayItemController::menu_event_group(
            &self.zbus_connection,
            &self.bus_name.get(),
            self.menu_path.get().as_str(),
            dbus_events,
        )
        .await
    }

    /// Gets a single property from a single menu item.
    ///
    /// Primarily useful for debugging. For production use, prefer getting
    /// properties from the MenuItem structure.
    ///
    /// # Arguments
    /// * `id` - Menu item ID
    /// * `property` - Property name
    ///
    /// # Returns
    /// * Property value
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus call fails or the menu is unreachable.
    pub async fn menu_get_property(&self, id: i32, property: &str) -> Result<OwnedValue, Error> {
        TrayItemController::menu_get_property(
            &self.zbus_connection,
            &self.bus_name.get(),
            self.menu_path.get().as_str(),
            id,
            property,
        )
        .await
    }

    /// Gets properties for multiple menu items.
    ///
    /// # Arguments
    /// * `ids` - Menu item IDs to query (empty = all items)
    /// * `property_names` - Property names to retrieve (empty = all properties)
    ///
    /// # Returns
    /// * Array of (id, properties) tuples
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus call fails or the menu is unreachable.
    pub async fn menu_get_group_properties(
        &self,
        ids: Vec<i32>,
        property_names: Vec<String>,
    ) -> Result<RawMenuItemsPropsList, Error> {
        TrayItemController::menu_get_group_properties(
            &self.zbus_connection,
            &self.bus_name.get(),
            self.menu_path.get().as_str(),
            ids,
            property_names,
        )
        .await
    }

    /// The item has a new title: the graphical representation should read it again immediately.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn new_title_signal(&self) -> Result<impl Stream<Item = ()>, Error> {
        let bus_name = &self.bus_name.get();
        let id = Self::parse_service_identifier(bus_name);
        let proxy = StatusNotifierItemProxy::builder(&self.zbus_connection)
            .destination(id.service)?
            .path(id.path)?
            .build()
            .await?;
        let stream = proxy.receive_new_title().await?;

        Ok(stream.filter_map(|_signal| async move { Some(()) }))
    }

    /// The item has a new icon: the graphical representation should read it again immediately.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn new_icon_signal(&self) -> Result<impl Stream<Item = ()>, Error> {
        let bus_name = &self.bus_name.get();
        let id = Self::parse_service_identifier(bus_name);
        let proxy = StatusNotifierItemProxy::builder(&self.zbus_connection)
            .destination(id.service)?
            .path(id.path)?
            .build()
            .await?;
        let stream = proxy.receive_new_icon().await?;

        Ok(stream.filter_map(|_signal| async move { Some(()) }))
    }

    /// The item has a new attention icon: the graphical representation should read it again
    /// immediately.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn new_attention_icon_signal(&self) -> Result<impl Stream<Item = ()>, Error> {
        let bus_name = &self.bus_name.get();
        let id = Self::parse_service_identifier(bus_name);
        let proxy = StatusNotifierItemProxy::builder(&self.zbus_connection)
            .destination(id.service)?
            .path(id.path)?
            .build()
            .await?;
        let stream = proxy.receive_new_attention_icon().await?;

        Ok(stream.filter_map(|_signal| async move { Some(()) }))
    }

    /// The item has a new overlay icon: the graphical representation should read it again
    /// immediately.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn new_overlay_icon_signal(&self) -> Result<impl Stream<Item = ()>, Error> {
        let bus_name = &self.bus_name.get();
        let id = Self::parse_service_identifier(bus_name);
        let proxy = StatusNotifierItemProxy::builder(&self.zbus_connection)
            .destination(id.service)?
            .path(id.path)?
            .build()
            .await?;
        let stream = proxy.receive_new_overlay_icon().await?;

        Ok(stream.filter_map(|_signal| async move { Some(()) }))
    }

    /// The item has a new tooltip: the graphical representation should read it again immediately.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn new_tool_tip_signal(&self) -> Result<impl Stream<Item = ()>, Error> {
        let bus_name = &self.bus_name.get();
        let id = Self::parse_service_identifier(bus_name);
        let proxy = StatusNotifierItemProxy::builder(&self.zbus_connection)
            .destination(id.service)?
            .path(id.path)?
            .build()
            .await?;
        let stream = proxy.receive_new_tool_tip().await?;

        Ok(stream.filter_map(|_signal| async move { Some(()) }))
    }

    /// The item has a new status, that is passed as an argument of the signal.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn new_status_signal(&self) -> Result<impl Stream<Item = Status>, Error> {
        let bus_name = &self.bus_name.get();
        let id = Self::parse_service_identifier(bus_name);
        let proxy = StatusNotifierItemProxy::builder(&self.zbus_connection)
            .destination(id.service)?
            .path(id.path)?
            .build()
            .await?;
        let stream = proxy.receive_new_status().await?;

        Ok(stream.filter_map(|signal| async move {
            signal
                .args()
                .ok()
                .map(|args| Status::from(args.status.as_str()))
        }))
    }

    /// Parse a service identifier into service name and object path.
    ///
    /// Handles two formats:
    /// - Bus name only: "org.kde.StatusNotifierItem-4077-1" -> uses default path
    /// - Bus name with path: ":1.234/StatusNotifierItem" -> splits at /
    fn parse_service_identifier(bus_name: &str) -> ServiceIdentifier<'_> {
        if let Some(slash_pos) = bus_name.find('/') {
            ServiceIdentifier {
                service: &bus_name[..slash_pos],
                path: &bus_name[slash_pos..],
            }
        } else {
            ServiceIdentifier {
                service: bus_name,
                path: "/StatusNotifierItem",
            }
        }
    }

    #[instrument(skip(connection), fields(bus_name = %bus_name), err)]
    async fn fetch_properties(
        connection: &Connection,
        bus_name: &str,
    ) -> Result<TrayItemProperties, Error> {
        let service_id = Self::parse_service_identifier(bus_name);

        let item_proxy = StatusNotifierItemProxy::builder(connection)
            .destination(service_id.service)?
            .path(service_id.path)?
            .build()
            .await?;

        let id = item_proxy.id().await;
        let title = item_proxy.title().await;
        let category = item_proxy.category().await.unwrap_or_default();
        let status = item_proxy.status().await.unwrap_or_default();
        let window_id = item_proxy.window_id().await;
        let item_is_menu = item_proxy.item_is_menu().await;
        let icon_name = item_proxy.icon_name().await.ok();
        let icon_pixmap = item_proxy.icon_pixmap().await;
        let overlay_icon_name = item_proxy.overlay_icon_name().await.ok();
        let overlay_icon_pixmap = item_proxy.overlay_icon_pixmap().await;
        let attention_icon_name = item_proxy.attention_icon_name().await.ok();
        let attention_icon_pixmap = item_proxy.attention_icon_pixmap().await;
        let attention_movie_name = item_proxy.attention_movie_name().await.ok();
        let icon_theme_path = item_proxy.icon_theme_path().await.ok();
        let tooltip = item_proxy.tool_tip().await;
        let menu_path = item_proxy.menu().await.unwrap_or_default();

        let service = service_id.service.to_string();
        let menu_proxy = DBusMenuProxy::builder(connection)
            .destination(service)?
            .path(menu_path.clone())?
            .cache_properties(CacheProperties::No)
            .build()
            .await?;

        const PARENT_ID: i32 = 0;
        const RECURSION_DEPTH: i32 = -1;
        let menu_item = menu_proxy
            .get_layout(PARENT_ID, RECURSION_DEPTH, vec![])
            .await
            .ok();

        Ok(TrayItemProperties {
            id: unwrap_string!(id),
            title: unwrap_string!(title),
            category: Category::from(category.as_str()),
            status: Status::from(status.as_str()),
            window_id: unwrap_u32!(window_id),
            item_is_menu: unwrap_bool!(item_is_menu),
            icon_name,
            icon_pixmap: icon_pixmap
                .unwrap_or_default()
                .into_iter()
                .map(IconPixmap::from)
                .collect(),
            overlay_icon_name,
            overlay_icon_pixmap: overlay_icon_pixmap
                .unwrap_or_default()
                .into_iter()
                .map(IconPixmap::from)
                .collect(),
            attention_icon_name,
            attention_icon_pixmap: attention_icon_pixmap
                .unwrap_or_default()
                .into_iter()
                .map(IconPixmap::from)
                .collect(),
            attention_movie_name,
            icon_theme_path,
            tooltip: tooltip.map(Tooltip::from).unwrap_or_default(),
            menu_path,
            menu: menu_item,
        })
    }

    fn from_properties(
        props: TrayItemProperties,
        connection: Connection,
        service: String,
        cancellation_token: Option<CancellationToken>,
    ) -> Self {
        let menu = props.menu.map(MenuItem::from);

        Self {
            zbus_connection: connection,
            cancellation_token,
            bus_name: Property::new(service),
            id: Property::new(props.id),
            title: Property::new(props.title),
            category: Property::new(props.category),
            status: Property::new(props.status),
            window_id: Property::new(props.window_id),
            item_is_menu: Property::new(props.item_is_menu),
            icon_name: Property::new(props.icon_name),
            icon_pixmap: Property::new(props.icon_pixmap),
            overlay_icon_name: Property::new(props.overlay_icon_name),
            overlay_icon_pixmap: Property::new(props.overlay_icon_pixmap),
            attention_icon_name: Property::new(props.attention_icon_name),
            attention_icon_pixmap: Property::new(props.attention_icon_pixmap),
            attention_movie_name: Property::new(props.attention_movie_name),
            icon_theme_path: Property::new(props.icon_theme_path),
            tooltip: Property::new(props.tooltip),
            menu: Property::new(menu),
            menu_path: Property::new(props.menu_path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_service_identifier_with_slash_splits_correctly() {
        let id = TrayItem::parse_service_identifier(":1.234/StatusNotifierItem");

        assert_eq!(id.service, ":1.234");
        assert_eq!(id.path, "/StatusNotifierItem");
    }

    #[test]
    fn parse_service_identifier_without_slash_uses_default_path() {
        let id = TrayItem::parse_service_identifier("org.kde.StatusNotifierItem-4077-1");

        assert_eq!(id.service, "org.kde.StatusNotifierItem-4077-1");
        assert_eq!(id.path, "/StatusNotifierItem");
    }

    #[test]
    fn parse_service_identifier_with_multiple_slashes_splits_at_first() {
        let id = TrayItem::parse_service_identifier(":1.234/some/nested/path");

        assert_eq!(id.service, ":1.234");
        assert_eq!(id.path, "/some/nested/path");
    }
}
