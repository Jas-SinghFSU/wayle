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
    let Some(wallpaper) = services.wallpaper.clone() else {
        return;
    };

    let styling = &services.config.config().styling;

    let theme_provider = styling.theme_provider.clone();
    tokio::spawn(async move {
        let mut stream = theme_provider.watch();
        while let Some(provider) = stream.next().await {
            wallpaper.color_extractor.set(map_to_extractor(provider));
        }
    });

    let wallpaper = services.wallpaper.clone().unwrap();
    let theming_monitor = styling.theming_monitor.clone();
    tokio::spawn(async move {
        let mut stream = theming_monitor.watch();
        while let Some(monitor) = stream.next().await {
            let opt = if monitor.is_empty() {
                None
            } else {
                Some(monitor)
            };
            wallpaper.set_theming_monitor(opt);
        }
    });
}
