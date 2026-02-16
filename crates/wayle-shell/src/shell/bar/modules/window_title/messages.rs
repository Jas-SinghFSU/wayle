use std::{rc::Rc, sync::Arc};

use wayle_config::ConfigService;
use wayle_hyprland::HyprlandService;
use wayle_widgets::prelude::BarSettings;

use crate::shell::bar::dropdowns::DropdownRegistry;

pub(crate) struct WindowTitleInit {
    pub settings: BarSettings,
    pub hyprland: Option<Arc<HyprlandService>>,
    pub config: Arc<ConfigService>,
    pub dropdowns: Rc<DropdownRegistry>,
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
