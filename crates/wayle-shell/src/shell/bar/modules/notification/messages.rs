use wayle_widgets::prelude::BarSettings;

pub struct NotificationInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum NotificationMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub enum NotificationCmd {
    NotificationsChanged(usize),
    DndChanged(bool),
    IconConfigChanged,
}
