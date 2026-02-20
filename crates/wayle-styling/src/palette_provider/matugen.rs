use std::fs;

use serde::Deserialize;
use wayle_config::infrastructure::{paths::ConfigPaths, themes::Palette};

use super::color;
use crate::{Error, palette_provider::PaletteProvider};

pub(crate) struct MatugenProvider;

impl PaletteProvider for MatugenProvider {
    fn load() -> Result<Palette, Error> {
        let path = ConfigPaths::matugen_colors().map_err(|_| {
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "cannot determine matugen colors path",
            ))
        })?;

        if !path.exists() {
            return Err(Error::PaletteNotFound(path));
        }

        let content = fs::read_to_string(&path)?;
        let output: MatugenOutput = serde_json::from_str(&content)?;

        Ok(output.into_palette())
    }
}

#[derive(Deserialize)]
struct MatugenOutput {
    colors: MaterialColors,
}

#[derive(Deserialize)]
struct MaterialColors {
    background: ColorVariants,
    on_background: ColorVariants,
    on_surface_variant: ColorVariants,
    primary: ColorVariants,
    secondary: ColorVariants,
    tertiary: ColorVariants,
    error: ColorVariants,
}

#[derive(Deserialize)]
struct ColorVariants {
    dark: ColorValue,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ColorValue {
    Plain(String),
    Nested { color: String },
}

impl MatugenOutput {
    fn into_palette(self) -> Palette {
        let colors = self.colors;
        let bg = colors.background.dark.as_color();

        Palette {
            bg: color::lighten(&bg, -0.04),
            surface: bg,
            elevated: color::lighten(&colors.background.dark.as_color(), 0.04),
            fg: colors.on_background.dark.as_color(),
            fg_muted: colors.on_surface_variant.dark.as_color(),
            primary: colors.primary.dark.as_color(),
            red: colors.error.dark.as_color(),
            yellow: colors.tertiary.dark.as_color(),
            green: colors.secondary.dark.as_color(),
            blue: colors.primary.dark.as_color(),
        }
    }
}

impl ColorValue {
    fn as_color(&self) -> String {
        match self {
            Self::Plain(color) | Self::Nested { color } => color.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MatugenOutput;

    const OLD_JSON: &str = r##"{
        "colors": {
            "background": { "dark": "#101112" },
            "surface": { "dark": "#202122" },
            "surface_variant": { "dark": "#303132" },
            "on_background": { "dark": "#f0f1f2" },
            "on_surface_variant": { "dark": "#a0a1a2" },
            "primary": { "dark": "#4090ff" },
            "secondary": { "dark": "#40ff90" },
            "tertiary": { "dark": "#ffcf40" },
            "error": { "dark": "#ff4040" }
        }
    }"##;

    const NEW_JSON: &str = r##"{
        "colors": {
            "background": { "dark": { "color": "#101112" } },
            "surface": { "dark": { "color": "#202122" } },
            "surface_variant": { "dark": { "color": "#303132" } },
            "on_background": { "dark": { "color": "#f0f1f2" } },
            "on_surface_variant": { "dark": { "color": "#a0a1a2" } },
            "primary": { "dark": { "color": "#4090ff" } },
            "secondary": { "dark": { "color": "#40ff90" } },
            "tertiary": { "dark": { "color": "#ffcf40" } },
            "error": { "dark": { "color": "#ff4040" } }
        }
    }"##;

    #[test]
    fn parses_old_matugen_shape() {
        let output: MatugenOutput = serde_json::from_str(OLD_JSON).unwrap();
        let palette = output.into_palette();
        assert_eq!(palette.surface, "#101112");
        assert_eq!(palette.primary, "#4090ff");
    }

    #[test]
    fn parses_new_matugen_shape() {
        let output: MatugenOutput = serde_json::from_str(NEW_JSON).unwrap();
        let palette = output.into_palette();
        assert_eq!(palette.surface, "#101112");
        assert_eq!(palette.primary, "#4090ff");
    }
}
