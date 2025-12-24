//! SCSS compilation and theming for Wayle.
//!
//! This crate handles runtime CSS generation from theme configuration.
//! It compiles embedded SCSS files with user-provided palette values.

mod error;

pub use error::Error;
use wayle_config::{infrastructure::themes::Palette, schemas::styling::FontConfig};

/// Compiles the complete stylesheet from palette, font, and scale inputs.
///
/// Generates SCSS variables from the inputs, combines them with the embedded
/// SCSS files (tokens, base styles), and compiles to CSS.
///
/// # Arguments
///
/// * `palette` - Color palette for the theme
/// * `fonts` - Font configuration for sans and mono families
/// * `scale` - Global UI scale multiplier (1.0 = default)
///
/// # Errors
///
/// Returns an error if SCSS compilation fails.
pub fn compile(palette: &Palette, fonts: &FontConfig, scale: f32) -> Result<String, Error> {
    let variables = format!(
        "{}\n{}\n{}\n",
        palette_to_scss(palette),
        fonts_to_scss(fonts),
        scale_to_scss(scale)
    );

    let tokens = include_str!("../scss/_tokens.scss");
    let base = include_str!("../scss/_base.scss");

    let full_scss = format!("{variables}\n{tokens}\n{base}");

    grass::from_string(full_scss, &grass::Options::default()).map_err(Error::Compilation)
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

fn fonts_to_scss(fonts: &FontConfig) -> String {
    format!(
        r#"$font-sans: "{}";
$font-mono: "{}";
"#,
        fonts.sans.get(),
        fonts.mono.get()
    )
}

fn scale_to_scss(scale: f32) -> String {
    format!("$global-scale: {};\n", scale)
}

#[cfg(test)]
mod tests {
    use wayle_config::infrastructure::themes::palettes;

    use super::*;

    #[test]
    fn compiled_css_loads_into_gtk4() {
        gtk4::init().unwrap();

        let palette = palettes::builtins().into_iter().next().unwrap();
        let fonts = FontConfig::default();
        let css = compile(&palette, &fonts, 1.0).unwrap();

        let provider = gtk4::CssProvider::new();
        provider.load_from_string(&css);
    }
}
