//! Synchronizes styling config with wallpaper service color extractor.

use futures::StreamExt;
use wayle_config::schemas::styling::{StylingConfig, ThemeProvider};
use wayle_wallpaper::{ColorExtractor, ColorExtractorConfig};

use crate::shell::ShellServices;

pub(crate) fn build_extractor_config(styling: &StylingConfig) -> ColorExtractorConfig {
    ColorExtractorConfig {
        tool: match styling.theme_provider.get() {
            ThemeProvider::Wayle => ColorExtractor::None,
            ThemeProvider::Matugen => ColorExtractor::Matugen,
            ThemeProvider::Pywal => ColorExtractor::Pywal,
            ThemeProvider::Wallust => ColorExtractor::Wallust,
        },
        matugen_scheme: styling.matugen_scheme.get().cli_value().into(),
        matugen_contrast: styling.matugen_contrast.get().value(),
        matugen_source_color: styling.matugen_source_color.get(),
        matugen_light: styling.matugen_light.get(),
        wallust_palette: styling.wallust_palette.get().config_value().into(),
        wallust_saturation: styling.wallust_saturation.get().value(),
        wallust_check_contrast: styling.wallust_check_contrast.get(),
        wallust_backend: styling.wallust_backend.get().config_value().into(),
        wallust_colorspace: styling.wallust_colorspace.get().config_value().into(),
        wallust_apply_globally: styling.wallust_apply_globally.get(),
        pywal_saturation: styling.pywal_saturation.get().value(),
        pywal_contrast: styling.pywal_contrast.get().value(),
        pywal_light: styling.pywal_light.get(),
        pywal_apply_globally: styling.pywal_apply_globally.get(),
    }
}

pub(crate) fn spawn(services: &ShellServices) {
    let Some(wallpaper_service) = services.wallpaper.clone() else {
        return;
    };

    let styling = services.config.config().styling.clone();

    let extractor_service = wallpaper_service.clone();
    let extractor_styling = styling.clone();
    tokio::spawn(async move {
        let mut theme_stream = extractor_styling.theme_provider.watch();
        let mut matugen_scheme_stream = extractor_styling.matugen_scheme.watch();
        let mut matugen_contrast_stream = extractor_styling.matugen_contrast.watch();
        let mut matugen_source_stream = extractor_styling.matugen_source_color.watch();
        let mut matugen_light_stream = extractor_styling.matugen_light.watch();
        let mut wallust_palette_stream = extractor_styling.wallust_palette.watch();
        let mut wallust_sat_stream = extractor_styling.wallust_saturation.watch();
        let mut wallust_contrast_stream = extractor_styling.wallust_check_contrast.watch();
        let mut wallust_backend_stream = extractor_styling.wallust_backend.watch();
        let mut wallust_colorspace_stream = extractor_styling.wallust_colorspace.watch();
        let mut wallust_global_stream = extractor_styling.wallust_apply_globally.watch();
        let mut pywal_sat_stream = extractor_styling.pywal_saturation.watch();
        let mut pywal_contrast_stream = extractor_styling.pywal_contrast.watch();
        let mut pywal_light_stream = extractor_styling.pywal_light.watch();
        let mut pywal_global_stream = extractor_styling.pywal_apply_globally.watch();

        loop {
            tokio::select! {
                Some(_) = theme_stream.next() => {}
                Some(_) = matugen_scheme_stream.next() => {}
                Some(_) = matugen_contrast_stream.next() => {}
                Some(_) = matugen_source_stream.next() => {}
                Some(_) = matugen_light_stream.next() => {}
                Some(_) = wallust_palette_stream.next() => {}
                Some(_) = wallust_sat_stream.next() => {}
                Some(_) = wallust_contrast_stream.next() => {}
                Some(_) = wallust_backend_stream.next() => {}
                Some(_) = wallust_colorspace_stream.next() => {}
                Some(_) = wallust_global_stream.next() => {}
                Some(_) = pywal_sat_stream.next() => {}
                Some(_) = pywal_contrast_stream.next() => {}
                Some(_) = pywal_light_stream.next() => {}
                Some(_) = pywal_global_stream.next() => {}
                else => break,
            }
            extractor_service
                .color_extractor
                .set(build_extractor_config(&extractor_styling));
        }
    });

    let theming_monitor = styling.theming_monitor.clone();
    tokio::spawn(async move {
        let mut stream = theming_monitor.watch();
        while let Some(monitor) = stream.next().await {
            let opt = if monitor.is_empty() {
                None
            } else {
                Some(monitor)
            };
            wallpaper_service.set_theming_monitor(opt);
        }
    });
}
