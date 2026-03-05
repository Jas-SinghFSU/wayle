use std::sync::Arc;

use wayle_cava::CavaService;
use wayle_config::ConfigService;
use wayle_wallpaper::WallpaperService;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct CavaInit {
    pub settings: BarSettings,
    pub config: Arc<ConfigService>,
    pub wallpaper: Option<Arc<WallpaperService>>,
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
