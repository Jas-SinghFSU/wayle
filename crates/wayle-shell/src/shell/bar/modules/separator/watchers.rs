use relm4::ComponentSender;
use wayle_common::{ConfigProperty, services, watch};
use wayle_config::ConfigService;

use super::{SeparatorCmd, SeparatorModule};

/// Spawns watchers for separator config and orientation changes.
pub(super) fn spawn_watchers(
    sender: &ComponentSender<SeparatorModule>,
    is_vertical: ConfigProperty<bool>,
) {
    let config_service = services::get::<ConfigService>();
    let sep_config = &config_service.config().modules.separator;
    let bar_config = &config_service.config().bar;
    let styling = &config_service.config().styling;

    let size = sep_config.size.clone();
    let length = sep_config.length.clone();
    let color = sep_config.color.clone();
    let scale = bar_config.scale.clone();
    let theme = styling.theme_provider.clone();

    watch!(
        sender,
        [
            size.watch(),
            length.watch(),
            color.watch(),
            scale.watch(),
            theme.watch()
        ],
        |out| {
            let _ = out.send(SeparatorCmd::StylingChanged);
        }
    );

    let is_vertical_prop = is_vertical.clone();
    watch!(sender, [is_vertical_prop.watch()], |out| {
        let _ = out.send(SeparatorCmd::OrientationChanged(is_vertical_prop.get()));
    });
}
