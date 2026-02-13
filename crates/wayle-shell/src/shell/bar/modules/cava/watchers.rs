use std::sync::Arc;

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_cava::CavaService;
use wayle_common::{ConfigProperty, watch, watch_cancellable, watchers::changes_stream};
use wayle_config::ConfigService;

use super::{CavaCmd, CavaModule};

pub(super) fn spawn_frame_watcher(
    sender: &ComponentSender<CavaModule>,
    cava: &Arc<CavaService>,
    token: CancellationToken,
) {
    let values = cava.values.clone();
    watch_cancellable!(sender, token, [values.watch()], |out| {
        let _ = out.send(CavaCmd::Frame(values.get()));
    });
}

pub(super) fn spawn_config_watchers(
    sender: &ComponentSender<CavaModule>,
    is_vertical: ConfigProperty<bool>,
    config: &Arc<ConfigService>,
) {
    let full_config = config.config();
    let cava_config = &full_config.modules.cava;
    let bar_config = &full_config.bar;
    let styling = &full_config.styling;

    let style = cava_config.style.clone();
    let direction = cava_config.direction.clone();
    let color = cava_config.color.clone();
    let button_bg_color = cava_config.button_bg_color.clone();
    let bar_width = cava_config.bar_width.clone();
    let bar_gap = cava_config.bar_gap.clone();
    let internal_padding = cava_config.internal_padding.clone();
    let border_show = cava_config.border_show.clone();
    let border_color = cava_config.border_color.clone();
    let scale = bar_config.scale.clone();
    let theme = styling.theme_provider.clone();
    let palette = styling.palette.clone();

    watch!(
        sender,
        [
            changes_stream(&style),
            changes_stream(&direction),
            changes_stream(&color),
            changes_stream(&button_bg_color),
            changes_stream(&bar_width),
            changes_stream(&bar_gap),
            changes_stream(&internal_padding),
            changes_stream(&border_show),
            changes_stream(&border_color),
            changes_stream(&scale),
            changes_stream(&theme),
            changes_stream(&palette)
        ],
        |out| {
            let _ = out.send(CavaCmd::StylingChanged);
        }
    );

    let bars = cava_config.bars.clone();
    let framerate = cava_config.framerate.clone();
    let stereo = cava_config.stereo.clone();
    let noise_reduction = cava_config.noise_reduction.clone();
    let monstercat = cava_config.monstercat.clone();
    let waves = cava_config.waves.clone();
    let low_cutoff = cava_config.low_cutoff.clone();
    let high_cutoff = cava_config.high_cutoff.clone();
    let input = cava_config.input.clone();
    let source = cava_config.source.clone();

    watch!(
        sender,
        [
            changes_stream(&bars),
            changes_stream(&framerate),
            changes_stream(&stereo),
            changes_stream(&noise_reduction),
            changes_stream(&monstercat),
            changes_stream(&waves),
            changes_stream(&low_cutoff),
            changes_stream(&high_cutoff),
            changes_stream(&input),
            changes_stream(&source)
        ],
        |out| {
            let _ = out.send(CavaCmd::ServiceConfigChanged);
        }
    );

    let is_vertical_prop = is_vertical.clone();
    watch!(sender, [changes_stream(&is_vertical_prop)], |out| {
        let _ = out.send(CavaCmd::OrientationChanged(is_vertical_prop.get()));
    });
}
