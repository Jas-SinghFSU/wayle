use relm4::ComponentSender;
use wayle_common::{services, watch};
use wayle_config::schemas::modules::NotificationConfig;
use wayle_notification::NotificationService;

use super::{NotificationModule, messages::NotificationCmd};

pub(super) fn spawn_watchers(
    sender: &ComponentSender<NotificationModule>,
    config: &NotificationConfig,
) {
    let notification_service = services::get::<NotificationService>();

    let notifications = notification_service.notifications.clone();
    watch!(sender, [notifications.watch()], |out| {
        let service = services::get::<NotificationService>();
        let count = service.notifications.get().len();
        let _ = out.send(NotificationCmd::NotificationsChanged(count));
    });

    let dnd = notification_service.dnd.clone();
    watch!(sender, [dnd.watch()], |out| {
        let service = services::get::<NotificationService>();
        let _ = out.send(NotificationCmd::DndChanged(service.dnd.get()));
    });

    let icon_name = config.icon_name.clone();
    let icon_unread = config.icon_unread.clone();
    let icon_dnd = config.icon_dnd.clone();
    watch!(
        sender,
        [icon_name.watch(), icon_unread.watch(), icon_dnd.watch()],
        |out| {
            let _ = out.send(NotificationCmd::IconConfigChanged);
        }
    );
}
