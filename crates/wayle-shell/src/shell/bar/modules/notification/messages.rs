use std::sync::Arc;

use wayle_config::ConfigService;
use wayle_notification::NotificationService;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct NotificationInit {
    pub settings: BarSettings,
    pub notification: Arc<NotificationService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum NotificationMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum NotificationCmd {
    NotificationsChanged(usize),
    DndChanged(bool),
    IconConfigChanged,
}
