//! CSS hot-reload watcher.

use relm4::ComponentSender;
use wayle_common::{services, watch, watchers::changes_stream};
use wayle_config::ConfigService;
use wayle_styling::compile;
use wayle_wallpaper::WallpaperService;

use crate::shell::{Shell, ShellCmd};

/// Spawns the CSS hot-reload watcher.
///
/// Watches styling config properties and color extraction events. When any
/// changes, recompiles CSS and sends the result to the shell component.
pub fn spawn(sender: &ComponentSender<Shell>) {
    let config = services::get::<ConfigService>().config().clone();
    let wallpaper_service = services::get::<WallpaperService>();

    watch!(sender,
        [
            changes_stream(&config.styling),
            wallpaper_service.watch_extraction(),
        ],
        move || compile_css(&config) => ShellCmd::CssRecompiled
    );
}

fn compile_css(config: &wayle_config::Config) -> Result<String, wayle_styling::Error> {
    let palette = config.styling.theme.palette.get();
    let fonts = &config.styling.fonts;
    let scale = config.styling.scale.get();
    let bar_scale = config.styling.bar_scale.get();
    let rounding = config.styling.rounding.get();
    let theme_provider = config.styling.theme_provider.get();

    compile(&palette, fonts, scale, bar_scale, rounding, theme_provider)
}
