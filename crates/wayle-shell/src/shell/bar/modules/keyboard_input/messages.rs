use wayle_widgets::prelude::BarSettings;

pub(crate) struct KeyboardInputInit {
    pub settings: BarSettings,
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
    UpdateTooltip(Option<String>),
}
