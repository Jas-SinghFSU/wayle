use std::{fmt, str::FromStr};

/// The urgency level of a notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Urgency {
    /// Low urgency. Server implementations may display the notification how they choose.
    Low = 0,
    /// Normal urgency. Server implementations may display the notification how they choose.
    Normal = 1,
    /// Critical urgency. Critical notifications should not automatically expire, as they are
    /// things that the user will most likely want to know about. They should only be closed
    /// when the user dismisses them, for example, by clicking on the notification.
    Critical = 2,
}

impl From<u8> for Urgency {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Low,
            2 => Self::Critical,
            _ => Self::Normal,
        }
    }
}

/// The reason a notification was closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ClosedReason {
    /// The notification expired.
    Expired = 1,
    /// The notification was dismissed by the user.
    DismissedByUser = 2,
    /// The notification was closed by a call to CloseNotification.
    Closed = 3,
    /// Undefined/reserved reasons.
    Unknown = 4,
}

impl From<u32> for ClosedReason {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Expired,
            2 => Self::DismissedByUser,
            3 => Self::Closed,
            _ => Self::Unknown,
        }
    }
}

/// Server capabilities as defined in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capabilities {
    /// Supports using icons instead of text for displaying actions. Using icons for actions
    /// must be enabled on a per-notification basis using the "action-icons" hint.
    ActionIcons,
    /// The server will provide the specified actions to the user. Even if this cap is missing,
    /// actions may still be specified by the client, however the server is free to ignore them.
    Actions,
    /// Supports body text. Some implementations may only show the summary
    /// (for instance, onscreen displays, marquee/scrollers)
    Body,
    /// The server supports hyperlinks in the notifications.
    BodyHyperlinks,
    /// The server supports images in the notifications.
    BodyImages,
    /// Supports markup in the body text. If marked up text is sent to a server that does not
    /// give this cap, the markup will show through as regular text so must be stripped clientside.
    BodyMarkup,
    /// The server will render an animation of all the frames in a given image array.
    /// The client may still specify multiple frames even if this cap and/or "icon-static"
    /// is missing, however the server is free to ignore them and use only the primary frame.
    IconMulti,
    /// Supports display of exactly 1 frame of any given image array. This value is mutually
    /// exclusive with "icon-multi", it is a protocol error for the server to specify both.
    IconStatic,
    /// The server supports persistence of notifications. Notifications will be retained until
    /// they are acknowledged or removed by the user or recalled by the sender. The presence
    /// of this capability allows clients to depend on the server to ensure a notification
    /// is seen and eliminate the need for the client to display a reminding function
    /// (such as a status icon) of its own.
    Persistence,
    /// The server supports sounds on notifications. If returned, the server must support
    /// the "sound-file" and "suppress-sound" hints.
    Sound,
    /// Vendor-specific capability.
    Vendor(String),
}

impl FromStr for Capabilities {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "action-icons" => Self::ActionIcons,
            "actions" => Self::Actions,
            "body" => Self::Body,
            "body-hyperlinks" => Self::BodyHyperlinks,
            "body-images" => Self::BodyImages,
            "body-markup" => Self::BodyMarkup,
            "icon-multi" => Self::IconMulti,
            "icon-static" => Self::IconStatic,
            "persistence" => Self::Persistence,
            "sound" => Self::Sound,
            s if s.starts_with("x-") => Self::Vendor(s.to_string()),
            _ => Self::Vendor(format!("x-unknown-{s}")),
        })
    }
}

impl Capabilities {
    /// Convert to string representation for D-Bus.
    pub fn as_str(&self) -> &str {
        match self {
            Self::ActionIcons => "action-icons",
            Self::Actions => "actions",
            Self::Body => "body",
            Self::BodyHyperlinks => "body-hyperlinks",
            Self::BodyImages => "body-images",
            Self::BodyMarkup => "body-markup",
            Self::IconMulti => "icon-multi",
            Self::IconStatic => "icon-static",
            Self::Persistence => "persistence",
            Self::Sound => "sound",
            Self::Vendor(s) => s,
        }
    }
}

impl fmt::Display for Capabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Standard notification categories.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Category {
    /// A generic audio or video call notification that doesn't fit into any other category.
    Call,
    /// An audio or video call was ended.
    CallEnded,
    /// A audio or video call is incoming.
    CallIncoming,
    /// An incoming audio or video call was not answered.
    CallUnanswered,
    /// A generic device-related notification that doesn't fit into any other category.
    Device,
    /// A device, such as a USB device, was added to the system.
    DeviceAdded,
    /// A device had some kind of error.
    DeviceError,
    /// A device, such as a USB device, was removed from the system.
    DeviceRemoved,
    /// A generic e-mail-related notification that doesn't fit into any other category.
    Email,
    /// A new e-mail notification.
    EmailArrived,
    /// A notification stating that an e-mail has bounced.
    EmailBounced,
    /// A generic instant message-related notification that doesn't fit into any other category.
    Im,
    /// An instant message error notification.
    ImError,
    /// A received instant message notification.
    ImReceived,
    /// A generic network notification that doesn't fit into any other category.
    Network,
    /// A network connection notification, such as successful sign-on to a network service.
    /// This should not be confused with device.added for new network devices.
    NetworkConnected,
    /// A network disconnected notification. This should not be confused with device.removed
    /// for disconnected network devices.
    NetworkDisconnected,
    /// A network-related or connection-related error.
    NetworkError,
    /// A generic presence change notification that doesn't fit into any other category,
    /// such as going away or idle.
    Presence,
    /// An offline presence change notification.
    PresenceOffline,
    /// An online presence change notification.
    PresenceOnline,
    /// A generic file transfer or download notification that doesn't fit into any other category.
    Transfer,
    /// A file transfer or download complete notification.
    TransferComplete,
    /// A file transfer or download error.
    TransferError,
    /// Vendor-specific category.
    Vendor(String),
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "call" => Self::Call,
            "call.ended" => Self::CallEnded,
            "call.incoming" => Self::CallIncoming,
            "call.unanswered" => Self::CallUnanswered,
            "device" => Self::Device,
            "device.added" => Self::DeviceAdded,
            "device.error" => Self::DeviceError,
            "device.removed" => Self::DeviceRemoved,
            "email" => Self::Email,
            "email.arrived" => Self::EmailArrived,
            "email.bounced" => Self::EmailBounced,
            "im" => Self::Im,
            "im.error" => Self::ImError,
            "im.received" => Self::ImReceived,
            "network" => Self::Network,
            "network.connected" => Self::NetworkConnected,
            "network.disconnected" => Self::NetworkDisconnected,
            "network.error" => Self::NetworkError,
            "presence" => Self::Presence,
            "presence.offline" => Self::PresenceOffline,
            "presence.online" => Self::PresenceOnline,
            "transfer" => Self::Transfer,
            "transfer.complete" => Self::TransferComplete,
            "transfer.error" => Self::TransferError,
            s if s.starts_with("x-") => Self::Vendor(s.to_string()),
            _ => Self::Vendor(format!("x-unknown-{s}")),
        })
    }
}

impl Category {
    /// Convert to string representation for hints.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Call => "call",
            Self::CallEnded => "call.ended",
            Self::CallIncoming => "call.incoming",
            Self::CallUnanswered => "call.unanswered",
            Self::Device => "device",
            Self::DeviceAdded => "device.added",
            Self::DeviceError => "device.error",
            Self::DeviceRemoved => "device.removed",
            Self::Email => "email",
            Self::EmailArrived => "email.arrived",
            Self::EmailBounced => "email.bounced",
            Self::Im => "im",
            Self::ImError => "im.error",
            Self::ImReceived => "im.received",
            Self::Network => "network",
            Self::NetworkConnected => "network.connected",
            Self::NetworkDisconnected => "network.disconnected",
            Self::NetworkError => "network.error",
            Self::Presence => "presence",
            Self::PresenceOffline => "presence.offline",
            Self::PresenceOnline => "presence.online",
            Self::Transfer => "transfer",
            Self::TransferComplete => "transfer.complete",
            Self::TransferError => "transfer.error",
            Self::Vendor(s) => s,
        }
    }
}

/// An action that can be invoked on a notification.
///
/// Actions are sent over as a list of pairs. Each even element in the list
/// (starting at index 0) represents the identifier for the action. Each odd
/// element in the list is the localized string that will be displayed to the user.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Action {
    /// The identifier for the action. The default action (usually invoked by clicking
    /// the notification) should have a key named "default".
    pub id: String,
    /// The localized string that will be displayed to the user.
    pub label: String,
}
