#![allow(clippy::too_many_arguments)]

use std::collections::HashMap;

use zbus::{proxy, zvariant::Value};

/// Standard `org.freedesktop.Notifications` D-Bus proxy.
///
/// Compatible with any desktop environment following the Desktop Notifications
/// Specification. Connects to the notification server at `/org/freedesktop/Notifications`.
#[proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
pub trait Notifications {
    /// Get the capabilities of the notification server.
    ///
    /// This call takes no parameters.
    /// It returns an array of strings. Each string describes an optional capability implemented
    /// by the server. The following values are defined by this spec:
    ///
    /// - "action-icons":    Supports using icons instead of text for displaying actions.
    ///                      Using icons for actions must be enabled on a per-notification
    ///                      basis using the "action-icons" hint.
    ///
    /// - "actions":         The server will provide the specified actions to the user. Even if
    ///                      this cap is missing, actions may still be specified by the client,
    ///                      however the server is free to ignore them.
    ///
    /// - "body":            Supports body text. Some implementations may only show the
    ///                      summary (for instance, onscreen displays, marquee/scrollers)
    ///
    /// - "body-hyperlinks": The server supports hyperlinks in the notifications.
    ///
    /// - "body-images":     The server supports images in the notifications.
    ///
    /// - "body-markup":     Supports markup in the body text. If marked up text is sent
    ///                      to a server that does not give this cap, the markup will show
    ///                      through as regular text so must be stripped clientside.
    ///
    /// - "icon-multi":      The server will render an animation of all the frames in a given
    ///                      image array. The client may still specify multiple frames even if
    ///                      this cap and/or "icon-static" is missing, however the server is
    ///                      free to ignore them and use only the primary frame.
    ///
    /// - "icon-static":     Supports display of exactly 1 frame of any given image array.
    ///                      This value is mutually exclusive with "icon-multi", it is a
    ///                      protocol error for the server to specify both.
    ///
    /// - "persistence":     The server supports persistence of notifications. Notifications
    ///                      will be retained until they are acknowledged or removed by the user
    ///                      or recalled by the sender. The presence of this capability allows
    ///                      clients to depend on the server to ensure a notification is seen
    ///                      and eliminate the need for the client to display a reminding function
    ///                      (such as a status icon) of its own.
    ///
    /// - "sound":           The server supports sounds on notifications. If returned, the server must
    ///                      support the "sound-file" and "sound-name" hints.
    ///
    /// New vendor-specific caps may be specified as long as they start with "x-vendor".
    /// For instance, "x-gnome-foo-cap". Capability names must not contain spaces. They are
    /// limited to alpha-numeric characters and dashes ("-").
    fn get_capabilities(&self) -> zbus::Result<Vec<String>>;

    /// Send a notification to the notification server.
    ///
    /// # Arguments
    ///
    /// * `app_name`       - The optional name of the application sending the notification.
    ///                      Can be blank.
    ///
    /// * `replaces_id`    - The optional notification ID that this notification replaces.
    ///                      The server must atomically (ie with no flicker or other visual cues)
    ///                      replace the given notification with this one. This allows clients to
    ///                      effectively modify the notification while it's active. A value of
    ///                      value of 0 means that this notification won't replace any existing
    ///                      notifications.
    ///
    /// * `app_icon`       - The optional program icon of the calling application. See Icons and
    ///                      Images. Can be an empty string, indicating no icon.
    ///
    /// * `summary`        - The summary text briefly describing the notification.
    ///
    /// * `body`           - The optional detailed body text. Can be empty.
    ///
    /// * `actions`        - Actions are sent over as a list of pairs. Each even element in the
    ///                      list (starting at index 0) represents the identifier for the action.
    ///                      Each odd element in the list is the localized string that will be
    ///                      displayed to the user.
    ///
    /// * `hints`          - Optional hints that can be passed to the server from the client program.
    ///                      Although clients and servers should never assume each other supports any
    ///                      specific hints, they can be used to pass along information, such as the
    ///                      process PID or window ID, that the server may be able to make use of.
    ///                      See Hints. Can be empty.
    ///
    /// * `expire_timeout` - The timeout time in milliseconds since the display of the
    ///                      notification at which the notification should automatically close.
    ///                      If -1, the notification's expiration time is dependent on the
    ///                      notification server's settings, and may vary for the type of
    ///                      notification. If 0, never expire.
    ///
    /// If replaces_id is 0, the return value is a UINT32 that represent the notification.
    /// It is unique, and will not be reused unless a MAXINT number of notifications have
    /// been generated. An acceptable implementation may just use an incrementing counter for
    /// the ID. The returned ID is always greater than zero. Servers must make sure not to
    /// return zero as an ID.
    ///
    /// If replaces_id is not 0, the returned value is the same value as replaces_id.
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<&str>,
        hints: HashMap<&str, Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;

    /// Causes a notification to be forcefully closed and removed from the user's view.
    /// It can be used, for example, in the event that what the notification pertains to
    /// is no longer relevant, or to cancel a notification with no expiration time.
    ///
    /// The NotificationClosed signal is emitted by this method.
    ///
    /// If the notification no longer exists, an empty D-BUS Error message is sent back.
    fn close_notification(&self, id: u32) -> zbus::Result<()>;

    /// Returns server information: name, vendor, version, and spec version.
    ///
    /// # Returns
    ///
    /// * `name`         - The product name of the server.
    /// * `vendor`       - The vendor name. For example, "KDE," "GNOME,"
    ///                    "freedesktop.org," or "Microsoft."
    /// * `version`      - The server's version number.
    /// * `spec_version` - The specification version the server is compliant with.
    fn get_server_information(&self) -> zbus::Result<(String, String, String, String)>;

    /// A completed notification is one that has timed out, or has been dismissed by the user.
    ///
    /// # Arguments
    ///
    /// * `id`     - The ID of the notification that was closed.
    /// * `reason` - The reason the notification was closed.
    ///              1 - The notification expired.
    ///              2 - The notification was dismissed by the user.
    ///              3 - The notification was closed by a call to CloseNotification.
    ///              4 - Undefined/reserved reasons.
    ///
    /// The ID specified in the signal is invalidated before the signal is sent and is
    /// no longer valid. Clients should remove any references to the ID.
    #[zbus(signal)]
    fn notification_closed(&self, id: u32, reason: u32) -> zbus::Result<()>;

    /// This signal is emitted when one of the following occurs:
    ///
    /// - The user performs some global "invoking" action upon a notification. For instance,
    ///   clicking somewhere on the notification itself.
    /// - The user invokes a specific action as specified in the original Notify request.
    ///   For instance, clicking on an action button.
    ///
    /// # Arguments
    ///
    /// * `id`         - The ID of the notification emitting the ActionInvoked signal.
    /// * `action_key` - The key of the action invoked. These match the keys sent over
    ///                  in the list of actions.
    #[zbus(signal)]
    fn action_invoked(&self, id: u32, action_key: String) -> zbus::Result<()>;

    /// This signal can be emitted before the ActionInvoked signal. It carries
    /// an activation token that can be used to activate a toplevel.
    ///
    /// # Arguments
    ///
    /// * `id`               - The ID of the notification emitting the ActivationToken signal.
    /// * `activation_token` - The activation token.
    #[zbus(signal)]
    fn activation_token(&self, id: u32, activation_token: String) -> zbus::Result<()>;
}
