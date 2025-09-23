mod controls;
mod monitoring;

use std::sync::Arc;

use controls::TrayItemController;
use tokio_util::sync::CancellationToken;
use zbus::{
    Connection,
    zvariant::{ObjectPath, OwnedObjectPath},
};

use crate::{
    services::{
        common::Property,
        systray::{
            error::Error,
            proxy::status_notifier_item::StatusNotifierItemProxy,
            types::{
                item::{Category, IconPixmap, Status, Tooltip},
                menu::MenuItem,
            },
        },
        traits::{ModelMonitoring, Reactive},
    },
    unwrap_bool, unwrap_string, unwrap_u32,
};

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
    pub menu: Property<MenuItem>,

    /// DBus path to an object which should implement the com.canonical.dbusmenu interface.
    pub menu_path: Property<OwnedObjectPath>,
}

impl PartialEq for TrayItem {
    fn eq(&self, other: &Self) -> bool {
        self.bus_name.get() == other.bus_name.get()
    }
}

pub(crate) struct TrayItemParams<'a> {
    pub connection: &'a Connection,
    pub service: String,
}

pub(crate) struct LiveTrayItemParams<'a> {
    pub connection: &'a Connection,
    pub service: String,
    pub cancellation_token: &'a CancellationToken,
}

impl Reactive for TrayItem {
    type Error = Error;
    type Context<'a> = TrayItemParams<'a>;
    type LiveContext<'a> = LiveTrayItemParams<'a>;

    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error> {
        let props = Self::fetch_properties(context.connection, &context.service).await?;
        Ok(Self::from_properties(
            props,
            context.connection.clone(),
            context.service.clone(),
            None,
        ))
    }

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

struct TrayItemProperties {
    id: String,
    title: String,
    category: Category,
    status: Status,
    window_id: u32,
    item_is_menu: bool,
    icon_name: Option<String>,
    icon_pixmap: Vec<IconPixmap>,
    overlay_icon_name: Option<String>,
    overlay_icon_pixmap: Vec<IconPixmap>,
    attention_icon_name: Option<String>,
    attention_icon_pixmap: Vec<IconPixmap>,
    attention_movie_name: Option<String>,
    icon_theme_path: Option<String>,
    tooltip: Tooltip,
    menu_path: OwnedObjectPath,
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
    pub async fn context_menu(&self, x: i32, y: i32) -> Result<(), Error> {
        TrayItemController::context_menu(&self.zbus_connection, &self.bus_name.get(), x, y).await
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
    pub async fn activate(&self, x: i32, y: i32) -> Result<(), Error> {
        TrayItemController::activate(&self.zbus_connection, &self.bus_name.get(), x, y).await
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
    pub async fn secondary_activate(&self, x: i32, y: i32) -> Result<(), Error> {
        TrayItemController::secondary_activate(&self.zbus_connection, &self.bus_name.get(), x, y)
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
    pub async fn scroll(&self, delta: i32, orientation: &str) -> Result<(), Error> {
        TrayItemController::scroll(
            &self.zbus_connection,
            &self.bus_name.get(),
            delta,
            orientation,
        )
        .await
    }

    async fn fetch_properties(
        connection: &Connection,
        bus_name: &str,
    ) -> Result<TrayItemProperties, Error> {
        let (service, path) = Self::parse_service_identifier(bus_name);
        let path = ObjectPath::try_from(path)?;

        let proxy = StatusNotifierItemProxy::builder(connection)
            .destination(service)?
            .path(path)?
            .build()
            .await?;

        let id = proxy.id().await;
        let title = proxy.title().await;
        let category = proxy.category().await.unwrap_or_default();
        let status = proxy.status().await.unwrap_or_default();
        let window_id = proxy.window_id().await;
        let item_is_menu = proxy.item_is_menu().await;
        let icon_name = proxy.icon_name().await.ok();
        let icon_pixmap = proxy.icon_pixmap().await;
        let overlay_icon_name = proxy.overlay_icon_name().await.ok();
        let overlay_icon_pixmap = proxy.overlay_icon_pixmap().await;
        let attention_icon_name = proxy.attention_icon_name().await.ok();
        let attention_icon_pixmap = proxy.attention_icon_pixmap().await;
        let attention_movie_name = proxy.attention_movie_name().await.ok();
        let icon_theme_path = proxy.icon_theme_path().await.ok();
        let tooltip = proxy.tool_tip().await;
        let menu_path = proxy.menu().await;

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
            menu_path: menu_path.unwrap_or_default(),
        })
    }

    fn from_properties(
        props: TrayItemProperties,
        connection: Connection,
        service: String,
        cancellation_token: Option<CancellationToken>,
    ) -> Self {
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
            menu: Property::new(MenuItem::new(0)),
            menu_path: Property::new(props.menu_path),
        }
    }

    /// Parse a service identifier into service name and object path.
    ///
    /// Handles two formats:
    /// - Bus name only: "org.kde.StatusNotifierItem-4077-1" -> uses default path
    /// - Bus name with path: ":1.234/StatusNotifierItem" -> splits at /
    fn parse_service_identifier(bus_name: &str) -> (&str, &str) {
        if let Some(slash_pos) = bus_name.find('/') {
            let service_part = &bus_name[..slash_pos];
            let path_part = &bus_name[slash_pos..];

            (service_part, path_part)
        } else {
            (bus_name, "/StatusNotifierItem")
        }
    }
}
