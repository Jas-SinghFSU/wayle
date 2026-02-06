use std::sync::Arc;

use wayle_config::ConfigService;
use wayle_hyprland::HyprlandService;
use wayle_widgets::prelude::BarSettings;

pub struct KeybindModeInit {
    pub settings: BarSettings,
    pub hyprland: Option<Arc<HyprlandService>>,
    pub config: Arc<ConfigService>,
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
