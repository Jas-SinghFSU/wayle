use std::sync::Arc;

use wayle_config::{ConfigService, schemas::modules::CustomModuleDefinition};
use wayle_widgets::prelude::BarSettings;

pub(crate) struct CustomInit {
    pub settings: BarSettings,
    pub config: Arc<ConfigService>,
    pub definition: CustomModuleDefinition,
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
