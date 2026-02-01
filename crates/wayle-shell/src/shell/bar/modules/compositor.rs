//! Compositor detection for compositor-dependent modules.

use std::env;

/// Detected Wayland compositor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Compositor {
    /// Hyprland compositor.
    Hyprland,
    /// Unknown or unsupported compositor.
    Unknown(String),
}

impl Compositor {
    /// Detects the running Wayland compositor.
    pub(crate) fn detect() -> Self {
        if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
            return Self::Hyprland;
        }

        let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
        Self::Unknown(desktop)
    }
}
