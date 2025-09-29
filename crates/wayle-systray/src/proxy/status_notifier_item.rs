use zbus::{Result, proxy, zvariant::OwnedObjectPath};

use crate::types::item::{RawPixmaps, RawTooltip};

/// Proxy for the org.kde.StatusNotifierItem interface.
///
/// Service: `[variable]` (e.g., org.kde.StatusNotifierItem-{PID}-{ID})
/// Interface: `org.kde.StatusNotifierItem`
/// Object path: `/StatusNotifierItem`
#[proxy(
    interface = "org.kde.StatusNotifierItem",
    default_path = "/StatusNotifierItem"
)]
pub(crate) trait StatusNotifierItem {
    /// Asks the status notifier item to show a context menu, this is typically a consequence of
    /// user input, such as mouse right click over the graphical representation of the item.
    ///
    /// The x and y parameters are in screen coordinates and is to be considered an hint to the
    /// item about where to show the context menu.
    fn context_menu(&self, x: i32, y: i32) -> Result<()>;

    /// Asks the status notifier item for activation, this is typically a consequence of user
    /// input, such as mouse left click over the graphical representation of the item. The
    /// application will perform any task is considered appropriate as an activation request.
    ///
    /// The x and y parameters are in screen coordinates and is to be considered an hint to the
    /// item where to show eventual windows (if any).
    fn activate(&self, x: i32, y: i32) -> Result<()>;

    /// Is to be considered a secondary and less important form of activation compared to
    /// Activate. This is typically a consequence of user input, such as mouse middle click over
    /// the graphical representation of the item. The application will perform any task is
    /// considered appropriate as an activation request.
    ///
    /// The x and y parameters are in screen coordinates and is to be considered an hint to the
    /// item where to show eventual windows (if any).
    fn secondary_activate(&self, x: i32, y: i32) -> Result<()>;

    /// The user asked for a scroll action. This is caused from input such as mouse wheel over
    /// the graphical representation of the item.
    ///
    /// The orientation parameter can be either horizontal or vertical.
    /// The amount of scroll is represented by delta: a positive value represents a scroll down
    /// or right, a negative value represents a scroll up or left.
    fn scroll(&self, delta: i32, orientation: &str) -> Result<()>;

    /// Describes the category of this item.
    ///
    /// The allowed values for the Category property are:
    /// - `ApplicationStatus`: The item describes the status of a generic application, for instance
    ///   the current state of a media player. In the case where the category of the item can not
    ///   be known, such as when the item is being proxied from another incompatible or emulated
    ///   system, ApplicationStatus can be used a sensible default fallback.
    /// - `Communications`: The item describes the status of communication oriented applications,
    ///   like an instant messenger or an email client.
    /// - `SystemServices`: The item describes services of the system not seen as a stand alone
    ///   application by the user, such as an indicator for the activity of a disk indexing
    ///   service.
    /// - `Hardware`: The item describes the state and control of a particular hardware, such as an
    ///   indicator of the battery charge or sound card volume control.
    #[zbus(property)]
    fn category(&self) -> Result<String>;

    /// It's a name that should be unique for this application and consistent between sessions,
    /// such as the application name itself.
    #[zbus(property)]
    fn id(&self) -> Result<String>;

    /// It's a name that describes the application, it can be more descriptive than Id.
    #[zbus(property)]
    fn title(&self) -> Result<String>;

    /// Describes the status of this item or of the associated application.
    ///
    /// The allowed values for the Status property are:
    /// - `Passive`: The item doesn't convey important information to the user, it can be considered
    ///   an "idle" status and is likely that visualizations will chose to hide it.
    /// - `Active`: The item is active, is more important that the item will be shown in some way to
    ///   the user.
    /// - `NeedsAttention`: The item carries really important information for the user, such as
    ///   battery charge running out and is wants to incentive the direct user intervention.
    ///   Visualizations should emphasize in some way the items with NeedsAttention status.
    #[zbus(property)]
    fn status(&self) -> Result<String>;

    /// It's the windowing-system dependent identifier for a window, the application can chose one
    /// of its windows to be available trough this property or just set 0 if it's not interested.
    #[zbus(property)]
    fn window_id(&self) -> Result<u32>;

    /// The StatusNotifierItem can carry an icon that can be used by the visualization to identify
    /// the item.
    ///
    /// An icon can either be identified by its Freedesktop-compliant icon name, carried by this
    /// property of by the icon data itself, carried by the property IconPixmap.
    ///
    /// Visualizations are encouraged to prefer icon names over icon pixmaps if both are available
    /// FIXME: still not very defined: could e the pixmap used as fallback if an icon name is
    /// not found?
    #[zbus(property)]
    fn icon_name(&self) -> Result<String>;

    /// Carries an ARGB32 binary representation of the icon, the format of icon data used in this
    /// specification is described in Section Icons.
    #[zbus(property)]
    fn icon_pixmap(&self) -> Result<RawPixmaps>;

    /// The Freedesktop-compliant name of an icon. This can be used by the visualization to
    /// indicate extra state information, for instance as an overlay for the main icon.
    #[zbus(property)]
    fn overlay_icon_name(&self) -> Result<String>;

    /// ARGB32 binary representation of the overlay icon described in the previous paragraph.
    #[zbus(property)]
    fn overlay_icon_pixmap(&self) -> Result<RawPixmaps>;

    /// The Freedesktop-compliant name of an icon. this can be used by the visualization to
    /// indicate that the item is in RequestingAttention state.
    #[zbus(property)]
    fn attention_icon_name(&self) -> Result<String>;

    /// ARGB32 binary representation of the requesting attention icon describe in the previous
    /// paragraph.
    #[zbus(property)]
    fn attention_icon_pixmap(&self) -> Result<RawPixmaps>;

    /// An item can also specify an animation associated to the RequestingAttention state. This
    /// should be either a Freedesktop-compliant icon name or a full path. The visualization can
    /// chose between the movie or AttentionIconPixmap (or using neither of those) at its
    /// discretion.
    #[zbus(property)]
    fn attention_movie_name(&self) -> Result<String>;

    /// Data structure that contains information for a tooltip, all fields are optional (e.g.
    /// the visualization is free to not display the DB.us;us provided tooltip icon).
    ///
    /// Fields:
    /// - STRING: Freedesktop-compliant name for an icon.
    /// - ARRAY(INT, INT, ARRAY(BYTE)): icon data
    /// - STRING: title for this tooltip
    /// - STRING: descriptive text for this tooltip. It can contain also a subset of the HTML
    ///   markup language, for a list of allowed tags see Section Markup.
    #[zbus(property)]
    fn tool_tip(&self) -> Result<RawTooltip>;

    /// The item only support the context menu, the visualization should prefer showing the menu
    /// or sending ContextMenu() instead of Activate()
    #[zbus(property)]
    fn item_is_menu(&self) -> Result<bool>;

    /// DBus path to an object which should implement the com.canonical.dbusmenu interface
    #[zbus(property)]
    fn menu(&self) -> Result<OwnedObjectPath>;

    /// An additional path to add to the theme search path to find the icons specified above.
    #[zbus(property)]
    fn icon_theme_path(&self) -> Result<String>;

    /// The item has a new title: the graphical representation should read it again immediately.
    #[zbus(signal)]
    fn new_title(&self) -> Result<()>;

    /// The item has a new icon: the graphical representation should read it again immediately.
    #[zbus(signal)]
    fn new_icon(&self) -> Result<()>;

    /// The item has a new attention icon: the graphical representation should read it again
    /// immediately.
    #[zbus(signal)]
    fn new_attention_icon(&self) -> Result<()>;

    /// The item has a new overlay icon: the graphical representation should read it again
    /// immediately.
    #[zbus(signal)]
    fn new_overlay_icon(&self) -> Result<()>;

    /// The item has a new tooltip: the graphical representation should read it again immediately.
    #[zbus(signal)]
    fn new_tool_tip(&self) -> Result<()>;

    /// The item has a new status, that is passed as an argument of the signal.
    #[zbus(signal)]
    fn new_status(&self, status: String) -> Result<()>;
}
