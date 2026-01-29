use wayle_common::ConfigProperty;

/// Initialization parameters for the separator module.
pub struct SeparatorInit {
    /// Whether the bar is vertical.
    pub is_vertical: ConfigProperty<bool>,
}

/// Commands sent to the separator component from async watchers.
#[derive(Debug)]
pub enum SeparatorCmd {
    /// Config styling properties changed (size, length, color).
    StylingChanged,
    /// Bar orientation changed.
    OrientationChanged(bool),
}
