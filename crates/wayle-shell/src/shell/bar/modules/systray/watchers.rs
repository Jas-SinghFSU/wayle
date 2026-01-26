use relm4::ComponentSender;
use wayle_common::{ConfigProperty, services, watch};
use wayle_config::ConfigService;
use wayle_systray::SystemTrayService;

use super::{SystrayCmd, SystrayModule};

pub(super) fn spawn_watchers(
    sender: &ComponentSender<SystrayModule>,
    is_vertical: &ConfigProperty<bool>,
) {
    let systray = services::get::<SystemTrayService>();
    let config_service = services::get::<ConfigService>();
    let systray_config = &config_service.config().modules.systray;
    let bar_config = &config_service.config().bar;

    let items = systray.items.clone();
    let blacklist = systray_config.blacklist.clone();
    let overrides = systray_config.overrides.clone();
    watch!(
        sender,
        [items.watch(), blacklist.watch(), overrides.watch()],
        |out| {
            let _ = out.send(SystrayCmd::ItemsChanged(items.get()));
        }
    );

    let item_gap = systray_config.item_gap.clone();
    let icon_scale = systray_config.icon_scale.clone();
    let internal_padding = systray_config.internal_padding.clone();
    let bar_scale = bar_config.scale.clone();
    watch!(
        sender,
        [
            item_gap.watch(),
            icon_scale.watch(),
            internal_padding.watch(),
            bar_scale.watch()
        ],
        |out| {
            let _ = out.send(SystrayCmd::StylingChanged);
        }
    );

    let is_vertical = is_vertical.clone();
    watch!(sender, [is_vertical.watch()], |out| {
        let _ = out.send(SystrayCmd::OrientationChanged(is_vertical.get()));
    });
}
