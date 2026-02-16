//! Synchronizes theme provider config with wallpaper service color extractor.

use futures::StreamExt;
use wayle_config::schemas::styling::ThemeProvider;
use wayle_wallpaper::types::ColorExtractor;

use crate::shell::ShellServices;

fn map_to_extractor(provider: ThemeProvider) -> ColorExtractor {
    match provider {
        ThemeProvider::Wayle => ColorExtractor::None,
        ThemeProvider::Matugen => ColorExtractor::Matugen,
        ThemeProvider::Pywal => ColorExtractor::Pywal,
        ThemeProvider::Wallust => ColorExtractor::Wallust,
    }
}

pub(crate) fn spawn(services: &ShellServices) {
    let Some(wallpaper_service) = services.wallpaper.clone() else {
        return;
    };

    let styling = &services.config.config().styling;

    let extractor_wallpaper_service = wallpaper_service.clone();
    let theme_provider = styling.theme_provider.clone();
    tokio::spawn(async move {
        let mut stream = theme_provider.watch();
        while let Some(provider) = stream.next().await {
            extractor_wallpaper_service
                .color_extractor
                .set(map_to_extractor(provider));
        }
    });

    let monitor_wallpaper_service = wallpaper_service;
    let theming_monitor = styling.theming_monitor.clone();
    tokio::spawn(async move {
        let mut stream = theming_monitor.watch();
        while let Some(monitor) = stream.next().await {
            let opt = if monitor.is_empty() {
                None
            } else {
                Some(monitor)
            };
            monitor_wallpaper_service.set_theming_monitor(opt);
        }
    });
}
