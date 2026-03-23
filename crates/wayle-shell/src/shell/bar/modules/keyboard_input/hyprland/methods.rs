use std::sync::Arc;

use relm4::{ComponentController, gtk};
use tokio::runtime::Handle;
use tracing::warn;
use wayle_hyprland::HyprlandService;
use wayle_widgets::{prelude::BarButtonInput, utils::force_window_resize};

use super::{HyprlandKeyboardInput, helpers};

impl HyprlandKeyboardInput {
    pub(super) fn update_label(&self, root: &gtk::Box) {
        let config = self.config.config();
        let format = config.modules.keyboard_input.format.get();
        let layout_alias_map = config.modules.keyboard_input.layout_alias_map.get();

        let label = helpers::format_label(&self.current_layout, &format, &layout_alias_map);
        self.bar_button.emit(BarButtonInput::SetLabel(label));
        force_window_resize(root);
    }
}

pub(super) fn initial_layout(hyprland: &Option<Arc<HyprlandService>>) -> String {
    let Some(hyprland) = hyprland else {
        warn!(
            service = "HyprlandService",
            "unavailable, using fallback layout"
        );
        return String::from("?");
    };

    let runtime = Handle::current();
    match runtime.block_on(hyprland.devices()) {
        Ok(devices) => helpers::main_keyboard_layout(&devices)
            .unwrap_or("?")
            .to_string(),
        Err(err) => {
            warn!(error = %err, "cannot get keyboard devices");
            String::from("?")
        }
    }
}
