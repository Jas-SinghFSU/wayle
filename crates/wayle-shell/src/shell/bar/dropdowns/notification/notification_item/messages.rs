use std::sync::Arc;

use wayle_notification::core::notification::Notification;

use crate::shell::notification_popup::helpers::ResolvedIcon;

pub(crate) struct NotificationItemInit {
    pub notification: Arc<Notification>,
    pub resolved_icon: ResolvedIcon,
}

#[derive(Debug)]
pub(crate) enum NotificationItemInput {
    RefreshTime,
}

#[derive(Debug)]
pub(crate) enum NotificationItemOutput {
    Dismissed(u32),
}
