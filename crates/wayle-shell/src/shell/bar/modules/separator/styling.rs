use std::sync::Arc;

#[allow(deprecated)]
use gtk4::prelude::StyleContextExt;
use gtk4::prelude::WidgetExt;
use relm4::gtk;
use wayle_config::{ConfigService, schemas::styling::ThemeProvider};
use wayle_widgets::styling::resolve_color;

const REM_BASE: f32 = 16.0;

fn rem_to_px_rounded(rem: f32, scale: f32) -> i32 {
    (rem * scale * REM_BASE).round() as i32
}

pub(super) fn init_css_provider(widget: &impl WidgetExt, provider: &gtk::CssProvider) {
    #[allow(deprecated)]
    widget
        .style_context()
        .add_provider(provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
}

pub(super) fn apply_styling(
    provider: &gtk::CssProvider,
    is_vertical: bool,
    config_service: &Arc<ConfigService>,
) {
    let full_config = config_service.config();
    let config = &full_config.modules.separator;
    let bar_config = &full_config.bar;
    let styling = &full_config.styling;

    let is_wayle_theme = matches!(styling.theme_provider.get(), ThemeProvider::Wayle);
    let color = resolve_color(&config.color, is_wayle_theme);

    let scale = bar_config.scale.get().value();
    let size_px = config.size.get() as i32;
    let length_px = rem_to_px_rounded(config.length.get().value(), scale);

    let (width, height) = if is_vertical {
        (length_px, size_px)
    } else {
        (size_px, length_px)
    };

    let css = format!(
        "separator {{ background-color: {color}; min-width: {width}px; min-height: {height}px; }}"
    );
    provider.load_from_string(&css);
}
