use wayle_common::ConfigProperty;

pub(crate) struct SeparatorInit {
    pub is_vertical: ConfigProperty<bool>,
}

#[derive(Debug)]
pub(crate) enum SeparatorCmd {
    StylingChanged,
    OrientationChanged(bool),
}
