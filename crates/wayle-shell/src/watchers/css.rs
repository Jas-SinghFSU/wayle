//! CSS hot-reload watcher.

use futures::StreamExt;
use relm4::ComponentSender;
use wayle_common::{services, watch, watchers::changes_stream};
use wayle_config::{ConfigService, schemas::styling::ThemeProvider};
use wayle_styling::compile;
use wayle_wallpaper::WallpaperService;

use crate::shell::{Shell, ShellCmd, ShellInput};

/// Spawns the CSS hot-reload watcher.
///
/// Watches styling config properties and color extraction events. Recompiles
/// CSS only when switching to Wayle provider or after extraction completes.
pub fn spawn(sender: &ComponentSender<Shell>) {
    let config = services::get::<ConfigService>().config().clone();
    let wallpaper_service = services::get::<WallpaperService>();

    if let Ok(css) = compile_css(&config) {
        sender.input_sender().send(ShellInput::ReloadCss(css)).ok();
    }

    let palette_stream = changes_stream(&config.styling.palette);
    let general_stream = changes_stream(&config.general);
    let bar_stream = changes_stream(&config.bar);
    let global_scale_stream = config.styling.scale.watch();
    let global_rounding_stream = config.styling.rounding.watch();

    let theme_provider_stream = config
        .styling
        .theme_provider
        .watch()
        .filter(|provider| std::future::ready(*provider == ThemeProvider::Wayle));

    let extraction_stream = wallpaper_service.watch_extraction();

    let config_clone = config.clone();
    watch!(sender,
        [
            palette_stream,
            general_stream,
            bar_stream,
            global_scale_stream,
            global_rounding_stream,
            theme_provider_stream,
            extraction_stream,
        ],
        move || compile_css(&config_clone) => ShellCmd::CssRecompiled
    );
}

fn compile_css(config: &wayle_config::Config) -> Result<String, wayle_styling::Error> {
    let palette = config.styling.palette();

    compile(&palette, &config.general, &config.bar, &config.styling)
}
