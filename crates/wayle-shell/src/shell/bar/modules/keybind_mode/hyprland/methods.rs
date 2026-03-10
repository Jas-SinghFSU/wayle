use std::sync::Arc;

use relm4::{ComponentController, gtk, gtk::prelude::*};
use tracing::warn;
use wayle_hyprland::HyprlandService;
use wayle_widgets::{prelude::BarButtonInput, utils::force_window_resize};

use super::{HyprlandKeybindMode, helpers};
use crate::i18n::t;

impl HyprlandKeybindMode {
    pub(super) fn update_display(&self, format: &str, root: &gtk::Box) {
        let auto_hide = self.config.config().modules.keybind_mode.auto_hide.get();

        let label = helpers::format_label(format, &self.current_mode);
        self.bar_button.emit(BarButtonInput::SetLabel(label));

        let visible = helpers::compute_visibility(&self.current_mode, auto_hide);
        if let Some(parent) = root.parent() {
            parent.set_visible(visible);
        }

        force_window_resize(root);
    }

    pub(super) fn initial_mode(hyprland: &Option<Arc<HyprlandService>>) -> String {
        let Some(hyprland) = hyprland else {
            warn!(
                service = "HyprlandService",
                "unavailable, using default mode"
            );
            return t!("bar-keybind-mode-default");
        };

        let runtime = tokio::runtime::Handle::current();
        match runtime.block_on(hyprland.submap()) {
            Ok(mode) => mode,
            Err(err) => {
                warn!(error = %err, "cannot get current keybind mode");
                t!("bar-keybind-mode-default")
            }
        }
    }
}
