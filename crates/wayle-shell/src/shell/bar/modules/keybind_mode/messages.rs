use wayle_widgets::prelude::BarSettings;

pub struct KeybindModeInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum KeybindModeMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub enum KeybindModeCmd {
    ModeChanged { name: String, format: String },
    FormatChanged,
    AutoHideChanged,
    UpdateIcon(String),
}
