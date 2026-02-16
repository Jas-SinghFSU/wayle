use std::{rc::Rc, sync::Arc};

use wayle_config::ConfigService;
use wayle_widgets::prelude::BarSettings;

use crate::shell::bar::dropdowns::DropdownRegistry;

pub(crate) struct WorldClockInit {
    pub settings: BarSettings,
    pub config: Arc<ConfigService>,
    pub dropdowns: Rc<DropdownRegistry>,
}

#[derive(Debug)]
pub(crate) enum WorldClockMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum WorldClockCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
