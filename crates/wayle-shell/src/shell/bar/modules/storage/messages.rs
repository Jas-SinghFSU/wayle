use wayle_widgets::prelude::BarSettings;

pub struct StorageInit {
    pub settings: BarSettings,
}

#[derive(Debug)]
pub enum StorageMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub enum StorageCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
