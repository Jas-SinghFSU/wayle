use std::{rc::Rc, sync::Arc};

use wayle_cava::CavaService;
use wayle_config::ConfigService;
use wayle_wallpaper::WallpaperService;
use wayle_widgets::prelude::BarSettings;

use crate::shell::bar::dropdowns::DropdownRegistry;

pub(crate) struct CavaInit {
    pub settings: BarSettings,
    pub config: Arc<ConfigService>,
    pub wallpaper: Option<Arc<WallpaperService>>,
    pub dropdowns: Rc<DropdownRegistry>,
}

#[derive(Debug)]
pub(crate) enum CavaCmd {
    ServiceReady(Arc<CavaService>),
    ServiceFailed,
    Frame(Vec<f64>),
    StylingChanged,
    ServiceConfigChanged,
    OrientationChanged(bool),
}

#[derive(Debug)]
pub(crate) enum CavaMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}
