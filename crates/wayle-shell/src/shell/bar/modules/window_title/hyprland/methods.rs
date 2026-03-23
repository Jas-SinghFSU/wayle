use std::sync::Arc;

use relm4::{ComponentController, gtk};
use tokio::runtime::Handle;
use tracing::warn;
use wayle_hyprland::HyprlandService;
use wayle_widgets::{prelude::BarButtonInput, utils::force_window_resize};

use super::{
    HyprlandWindowTitle,
    helpers::{self, IconContext},
};
use crate::i18n::t;

impl HyprlandWindowTitle {
    pub(super) fn update_display(&self, format: &str, root: &gtk::Box) {
        let window_title = &self.config.config().modules.window_title;

        let label = helpers::format_label(format, &self.current_title, &self.current_class);
        let icon = helpers::resolve_icon(&IconContext {
            title: &self.current_title,
            class: &self.current_class,
            user_mappings: &window_title.icon_mappings.get(),
            fallback: &window_title.icon_name.get(),
        });

        self.bar_button.emit(BarButtonInput::SetLabel(label));
        self.bar_button.emit(BarButtonInput::SetIcon(icon));
        force_window_resize(root);
    }

    pub(super) fn update_label(&self, format: &str, root: &gtk::Box) {
        let label = helpers::format_label(format, &self.current_title, &self.current_class);
        self.bar_button.emit(BarButtonInput::SetLabel(label));
        force_window_resize(root);
    }
}

pub(super) fn initial_window(hyprland: &Option<Arc<HyprlandService>>) -> (String, String) {
    let fallback = || (t!("bar-window-title-empty"), t!("bar-window-title-empty"));

    let Some(hyprland) = hyprland else {
        warn!(
            service = "HyprlandService",
            "unavailable, using fallback window"
        );
        return fallback();
    };

    let runtime = Handle::current();
    match runtime.block_on(hyprland.active_window()) {
        Some(client) => (client.title.get(), client.class.get()),
        None => fallback(),
    }
}
