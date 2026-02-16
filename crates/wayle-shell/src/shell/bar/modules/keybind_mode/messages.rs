use std::{rc::Rc, sync::Arc};

use wayle_config::ConfigService;
use wayle_hyprland::HyprlandService;
use wayle_widgets::prelude::BarSettings;

use crate::shell::bar::dropdowns::DropdownRegistry;

pub struct KeybindModeInit {
    pub settings: BarSettings,
    pub hyprland: Option<Arc<HyprlandService>>,
    pub config: Arc<ConfigService>,
    pub dropdowns: Rc<DropdownRegistry>,
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
