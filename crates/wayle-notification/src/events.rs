use crate::{core::notification::Notification, types::ClosedReason};

#[derive(Clone)]
pub(crate) enum NotificationEvent {
    Add(Box<Notification>),
    Remove(u32, ClosedReason),
}
