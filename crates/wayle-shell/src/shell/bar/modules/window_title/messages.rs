use std::sync::Arc;

use wayle_config::ConfigService;
use wayle_hyprland::HyprlandService;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct WindowTitleInit {
    pub settings: BarSettings,
    pub hyprland: Option<Arc<HyprlandService>>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum WindowTitleMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum WindowTitleCmd {
    WindowChanged {
        title: String,
        class: String,
        format: String,
    },
    FormatChanged,
    IconConfigChanged,
}
