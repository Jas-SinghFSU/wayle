//! Synchronizes theme provider config with wallpaper service color extractor.

use futures::StreamExt;
use wayle_common::services;
use wayle_config::{ConfigService, schemas::styling::ThemeProvider};
use wayle_wallpaper::{WallpaperService, types::ColorExtractor};

fn map_to_extractor(provider: ThemeProvider) -> ColorExtractor {
    match provider {
        ThemeProvider::Wayle => ColorExtractor::None,
        ThemeProvider::Matugen => ColorExtractor::Matugen,
        ThemeProvider::Pywal => ColorExtractor::Pywal,
        ThemeProvider::Wallust => ColorExtractor::Wallust,
    }
}

pub(crate) fn spawn() {
    let config_service = services::get::<ConfigService>();
    let wallpaper = services::get::<WallpaperService>();
    let theme_provider = config_service.config().styling.theme_provider.clone();

    let initial = theme_provider.get();
    let extractor = map_to_extractor(initial);
    wallpaper.color_extractor.set(extractor);

    tokio::spawn(async move {
        let mut stream = theme_provider.watch();

        while let Some(provider) = stream.next().await {
            let extractor = map_to_extractor(provider);
            wallpaper.color_extractor.set(extractor);
        }
    });
}
