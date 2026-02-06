use std::sync::Arc;

use wayle_config::ConfigService;
use wayle_hyprland::HyprlandService;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct KeyboardInputInit {
    pub settings: BarSettings,
    pub hyprland: Option<Arc<HyprlandService>>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum KeyboardInputMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum KeyboardInputCmd {
    LayoutChanged { layout: String, format: String },
    FormatChanged,
    UpdateIcon(String),
}
