use std::{rc::Rc, sync::Arc};

use wayle_config::{ConfigService, schemas::modules::CustomModuleDefinition};
use wayle_widgets::prelude::BarSettings;

use crate::shell::bar::dropdowns::DropdownRegistry;

pub(crate) struct CustomInit {
    pub settings: BarSettings,
    pub config: Arc<ConfigService>,
    pub definition: CustomModuleDefinition,
    pub dropdowns: Rc<DropdownRegistry>,
}

#[derive(Debug)]
pub(crate) enum CustomMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum CustomCmd {
    PollTrigger,
    ScrollDebounceExpired,
    CommandCancelled,
    CommandOutput(String),
    WatchOutput(String),
    DefinitionChanged(Box<CustomModuleDefinition>),
    DefinitionRemoved,
}
