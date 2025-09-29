use crate::{core::notification::Notification, types::ClosedReason};

/// Events emitted by the notification daemon.
#[derive(Clone)]
pub(crate) enum NotificationEvent {
    Add(Box<Notification>),
    Remove(u32, ClosedReason),
}
