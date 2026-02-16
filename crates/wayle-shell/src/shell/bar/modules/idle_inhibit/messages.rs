use std::{rc::Rc, sync::Arc};

use wayle_config::ConfigService;
use wayle_widgets::prelude::BarSettings;

use crate::{services::idle_inhibit::IdleInhibitService, shell::bar::dropdowns::DropdownRegistry};

pub(crate) struct IdleInhibitInit {
    pub settings: BarSettings,
    pub idle_inhibit: Arc<IdleInhibitService>,
    pub config: Arc<ConfigService>,
    pub dropdowns: Rc<DropdownRegistry>,
}

#[derive(Debug)]
pub(crate) enum IdleInhibitMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum IdleInhibitCmd {
    ConfigChanged,
    StateChanged,
}
