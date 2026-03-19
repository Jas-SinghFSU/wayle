use std::sync::Arc;

use relm4::ComponentSender;
use wayle_common::watch;
use wayle_config::ConfigService;
use wayle_notification::NotificationService;

use super::{NotificationPopupHost, messages::PopupHostCmd};

pub(super) fn spawn(
    sender: &ComponentSender<NotificationPopupHost>,
    notification: &Arc<NotificationService>,
    config: &Arc<ConfigService>,
) {
    let popups = notification.popups.clone();
    watch!(sender, [popups.watch()], |out| {
        let _ = out.send(PopupHostCmd::PopupsChanged(popups.get()));
    });

    let full_config = config.config();
    let notif_config = full_config.modules.notification.clone();
    let position = notif_config.popup_position.clone();
    let margin_x = notif_config.popup_margin_x.clone();
    let margin_y = notif_config.popup_margin_y.clone();
    let gap = notif_config.popup_gap.clone();
    let max_visible = notif_config.popup_max_visible.clone();
    let stacking_order = notif_config.popup_stacking_order.clone();
    let duration = notif_config.popup_duration.clone();
    let monitor = notif_config.popup_monitor.clone();
    let icon_source = notif_config.icon_source.clone();
    let close_behavior = notif_config.popup_close_behavior.clone();
    let hover_pause = notif_config.popup_hover_pause.clone();
    let scale = full_config.styling.scale.clone();

    watch!(
        sender,
        [
            position.watch(),
            margin_x.watch(),
            margin_y.watch(),
            gap.watch(),
            max_visible.watch(),
            stacking_order.watch(),
            duration.watch(),
            monitor.watch(),
            icon_source.watch(),
            close_behavior.watch(),
            hover_pause.watch(),
            scale.watch(),
        ],
        |out| {
            let _ = out.send(PopupHostCmd::ConfigChanged);
        }
    );
}
