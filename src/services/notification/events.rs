use super::core::notification::Notification;

/// Events emitted by the notification daemon.
#[derive(Clone)]
pub(crate) enum NotificationEvent {
    Add(Box<Notification>),
    Remove(u32),
}
