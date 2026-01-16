//! SCSS compilation and theming for Wayle.
//!
//! Runtime CSS generation from theme configuration.
//! Compiles embedded SCSS files with user-provided palette values.

mod errors;
mod palette_provider;

use std::{fs, path::PathBuf};

pub use errors::Error;
use tracing::{error, info};
use wayle_config::{
    infrastructure::themes::Palette,
    schemas::{bar::BarConfig, general::GeneralConfig, styling::{RoundingLevel, ThemeProvider}},
};

fn scss_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scss")
}

/// Compiles the complete stylesheet from palette, font, scale, and rounding inputs.
///
/// # Errors
///
/// Returns an error if SCSS compilation fails.
pub fn compile(
    palette: &Palette,
    general: &GeneralConfig,
    bar: &BarConfig,
    scale: f32,
    rounding: RoundingLevel,
    theme_provider: ThemeProvider,
) -> Result<String, Error> {
    let resolved_palette = match resolve_palette(palette, &theme_provider) {
        Ok(palette) => palette,
        Err(e) => {
            error!(error = %e, provider = ?theme_provider, "cannot resolve palette from provider");
            info!("Falling back to Wayle styling");

            palette
        }
    };

    let variables = format!(
        "{}\n{}\n{}\n{}\n",
        palette_to_scss(resolved_palette),
        fonts_to_scss(general),
        scale_to_scss(scale, bar),
        rounding_to_scss(rounding)
    );

    let scss_path = scss_dir();
    let main_path = scss_path.join("main.scss");

    let main_content = fs::read_to_string(&main_path).map_err(Error::Io)?;
    let full_scss = main_content.replace("@import \"variables\";", &variables);

    let options = grass::Options::default().load_path(&scss_path);

    grass::from_string(&full_scss, &options).map_err(Error::Compilation)
}

fn resolve_palette<'a>(
    fallback: &'a Palette,
    theme_provider: &ThemeProvider,
) -> Result<&'a Palette, Error> {
    match theme_provider {
        ThemeProvider::Wayle => Ok(fallback),
        ThemeProvider::Matugen | ThemeProvider::Pywal | ThemeProvider::Wallust => {
            Err(Error::ProviderNotImplemented(*theme_provider))
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

fn scale_to_scss(scale: f32, bar: &BarConfig) -> String {
    format!(
        "$global-scale: {};\n\
         $bar-scale: {};\n\
         $bar-btn-icon-scale: {};\n\
         $bar-btn-icon-padding-scale: {};\n\
         $bar-btn-label-scale: {};\n\
         $bar-btn-label-padding-scale: {};\n\
         $bar-btn-gap-scale: {};\n",
        scale,
        bar.scale.get(),
        bar.button_icon_scale.get(),
        bar.button_icon_padding_scale.get(),
        bar.button_label_scale.get(),
        bar.button_label_padding_scale.get(),
        bar.button_gap_scale.get()
    )
}

fn rounding_to_scss(rounding: RoundingLevel) -> String {
    let global = rounding.to_css_values();
    let bar = rounding.to_bar_css_values();
    format!(
        "$rounding-element: {};\n$rounding-container: {};\n\
         $bar-rounding-element: {};\n$bar-rounding-container: {};\n",
        global.element, global.container, bar.element, bar.container
    )
}

#[cfg(test)]
mod tests {
    use wayle_config::{infrastructure::themes::palettes, schemas::bar::BarConfig};

    use super::*;

    #[test]
    fn compiled_css_loads_into_gtk4() {
        gtk4::init().unwrap();

        let palette = palettes::builtins().into_iter().next().unwrap();
        let general = GeneralConfig::default();
        let bar = BarConfig::default();
        let css = compile(
            &palette,
            &general,
            &bar,
            1.0,
            RoundingLevel::default(),
            ThemeProvider::default(),
        )
        .unwrap();

        let provider = gtk4::CssProvider::new();
        provider.load_from_string(&css);
    }

    #[test]
    fn debug_print_css() {
        let palette = palettes::builtins().into_iter().next().unwrap();
        let general = GeneralConfig::default();
        let bar = BarConfig::default();
        let css = compile(
            &palette,
            &general,
            &bar,
            1.0,
            RoundingLevel::default(),
            ThemeProvider::default(),
        )
        .unwrap();

        println!("\n=== COMPILED CSS ===\n{}\n=== END ===", css);
    }
}
