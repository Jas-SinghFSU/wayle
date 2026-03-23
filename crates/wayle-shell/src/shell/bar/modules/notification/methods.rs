use relm4::ComponentController;
use wayle_config::schemas::modules::NotificationConfig;
use wayle_widgets::prelude::BarButtonInput;

use super::{
    NotificationModule,
    helpers::{IconContext, format_label, select_icon},
};

impl NotificationModule {
    pub(super) fn update_display(&self, config: &NotificationConfig) {
        let icon_name = config.icon_name.get();
        let icon_unread = config.icon_unread.get();
        let icon_dnd = config.icon_dnd.get();

        let icon = select_icon(&IconContext {
            count: self.count,
            dnd: self.dnd,
            icon_name: &icon_name,
            icon_unread: &icon_unread,
            icon_dnd: &icon_dnd,
        });
        self.bar_button.emit(BarButtonInput::SetIcon(icon));

        let label = format_label(self.count);
        self.bar_button.emit(BarButtonInput::SetLabel(label));
    }
}
