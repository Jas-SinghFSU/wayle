use relm4::{ComponentController, gtk::prelude::*};
use tracing::warn;
use wayle_idle_inhibit::IdleInhibitor;
use wayle_widgets::prelude::BarButtonInput;

use super::{
    IdleInhibitModule,
    helpers::{self, LabelContext},
};

impl IdleInhibitModule {
    pub(super) fn sync_inhibitor(&mut self) {
        let should_be_active = self.state.active.get();
        let is_active = self.inhibitor.is_some();

        if should_be_active && !is_active {
            self.create_inhibitor();
        } else if !should_be_active && is_active {
            self.inhibitor.take();
        }
    }

    fn create_inhibitor(&mut self) {
        let widget = self.bar_button.widget();
        let Some(native) = widget.native() else {
            warn!("widget has no native surface");
            return;
        };
        let Some(gdk_surface) = native.surface() else {
            warn!("native has no surface");
            return;
        };
        let Some(inhibitor) = IdleInhibitor::new(&gdk_surface) else {
            warn!("failed to create idle inhibitor");
            return;
        };

        self.inhibitor = Some(inhibitor);
    }

    pub(super) fn update_display(
        &self,
        config: &wayle_config::schemas::modules::IdleInhibitConfig,
    ) {
        let active = self.state.active.get();

        let icon = helpers::select_icon(
            active,
            &config.icon_inactive.get(),
            &config.icon_active.get(),
        );
        self.bar_button.emit(BarButtonInput::SetIcon(icon));

        let label = helpers::build_label(
            &config.format.get(),
            &LabelContext {
                active,
                duration_mins: self.state.duration_mins.get(),
                remaining_secs: self.state.remaining_secs.get(),
            },
        );
        self.bar_button.emit(BarButtonInput::SetLabel(label));
    }
}
