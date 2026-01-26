//! SCSS compilation and theming for Wayle.
//!
//! Runtime CSS generation from theme configuration.
//! Compiles embedded SCSS files with user-provided palette values.

mod errors;
mod palette_provider;

use std::{fs, path::PathBuf};

pub use errors::Error;
use tracing::error;
use wayle_config::{
    infrastructure::themes::Palette,
    schemas::{
        bar::BarConfig,
        general::GeneralConfig,
        styling::{StylingConfig, ThemeProvider},
    },
};

/// Returns the SCSS source directory path.
///
/// Only useful during development for hot-reload watching.
pub fn scss_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scss")
}

/// Compiles the complete stylesheet from palette, font, and bar config.
///
/// # Errors
///
/// Returns an error if SCSS compilation fails.
pub fn compile(
    palette: &Palette,
    general: &GeneralConfig,
    bar: &BarConfig,
    styling: &StylingConfig,
) -> Result<String, Error> {
    let theme_provider = styling.theme_provider.get();
    let resolved_palette = resolve_palette(palette, &theme_provider);

    let variables = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        palette_to_scss(&resolved_palette),
        fonts_to_scss(general),
        global_scale_to_scss(styling),
        scale_to_scss(bar),
        rounding_to_scss(styling, bar)
    );

    let scss_path = scss_dir();
    let main_path = scss_path.join("main.scss");

    let main_content = fs::read_to_string(&main_path).map_err(Error::Io)?;
    let full_scss = main_content.replace("@import \"variables\";", &variables);

    let options = grass::Options::default().load_path(&scss_path);

    grass::from_string(&full_scss, &options).map_err(Error::Compilation)
}

fn resolve_palette(fallback: &Palette, theme_provider: &ThemeProvider) -> Palette {
    use palette_provider::PaletteProvider;

    match theme_provider {
        ThemeProvider::Wayle => fallback.clone(),
        ThemeProvider::Matugen => palette_provider::matugen::MatugenProvider::load()
            .unwrap_or_else(|e| {
                error!(error = %e, "matugen palette load failed");
                fallback.clone()
            }),
        ThemeProvider::Wallust => palette_provider::wallust::WallustProvider::load()
            .unwrap_or_else(|e| {
                error!(error = %e, "wallust palette load failed");
                fallback.clone()
            }),
        ThemeProvider::Pywal => {
            palette_provider::pywal::PywalProvider::load().unwrap_or_else(|e| {
                error!(error = %e, "pywal palette load failed");
                fallback.clone()
            })
        }
    }
}

fn palette_to_scss(palette: &Palette) -> String {
    format!(
        r#"$palette-bg: {};
$palette-surface: {};
$palette-elevated: {};
$palette-fg: {};
$palette-fg-muted: {};
$palette-primary: {};
$palette-red: {};
$palette-yellow: {};
$palette-green: {};
$palette-blue: {};
"#,
        palette.bg,
        palette.surface,
        palette.elevated,
        palette.fg,
        palette.fg_muted,
        palette.primary,
        palette.red,
        palette.yellow,
        palette.green,
        palette.blue
    )
}

fn fonts_to_scss(general: &GeneralConfig) -> String {
    format!(
        r#"$font-sans: "{}";
$font-mono: "{}";
"#,
        general.font_sans.get(),
        general.font_mono.get()
    )
}

fn global_scale_to_scss(styling: &StylingConfig) -> String {
    format!("$global-scale: {};\n", styling.scale.get())
}

fn scale_to_scss(bar: &BarConfig) -> String {
    format!(
        "$bar-scale: {};\n\
         $bar-btn-icon-scale: {};\n\
         $bar-btn-icon-padding-scale: {};\n\
         $bar-btn-label-scale: {};\n\
         $bar-btn-label-padding-scale: {};\n\
         $bar-btn-gap-scale: {};\n",
        bar.scale.get(),
        bar.button_icon_size.get(),
        bar.button_icon_padding.get(),
        bar.button_label_size.get(),
        bar.button_label_padding.get(),
        bar.button_gap.get()
    )
}

fn rounding_to_scss(styling: &StylingConfig, bar: &BarConfig) -> String {
    let global_rounding = styling.rounding.get();
    let bar_rounding = bar.rounding.get();
    let button_rounding = bar.button_rounding.get();
    let group_rounding = bar.button_group_rounding.get();
    let global = global_rounding.to_css_values();
    let bar_values = bar_rounding.to_bar_css_values();
    let bar_button_values = button_rounding.to_bar_element_css_values();
    let bar_group_values = group_rounding.to_bar_element_css_values();
    format!(
        "$rounding-element: {};\n\
        $rounding-container: {};\n\
        $bar-rounding-element: {};\n\
        $bar-rounding-container: {};\n\
        $bar-button-rounding-element: {};\n\
        $bar-group-rounding-element: {};\n",
        global.element,
        global.container,
        bar_values.element,
        bar_values.container,
        bar_button_values.element,
        bar_group_values.element
    )
}

#[cfg(test)]
mod tests {
    use wayle_config::{
        infrastructure::themes::palettes,
        schemas::{bar::BarConfig, styling::StylingConfig},
    };

    use super::*;

    #[test]
    fn compiled_css_loads_into_gtk4() {
        gtk4::init().unwrap();

        let theme = palettes::builtins().into_iter().next().unwrap();
        let general = GeneralConfig::default();
        let bar = BarConfig::default();
        let styling = StylingConfig::default();
        let css = compile(&theme.palette, &general, &bar, &styling).unwrap();

        let provider = gtk4::CssProvider::new();
        provider.load_from_string(&css);
    }

    #[test]
    fn debug_print_css() {
        let theme = palettes::builtins().into_iter().next().unwrap();
        let general = GeneralConfig::default();
        let bar = BarConfig::default();
        let styling = StylingConfig::default();
        let css = compile(&theme.palette, &general, &bar, &styling).unwrap();

        println!("\n=== COMPILED CSS ===\n{}\n=== END ===", css);
    }
}
