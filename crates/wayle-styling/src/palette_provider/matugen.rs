use std::fs;

use serde::Deserialize;
use wayle_config::infrastructure::{paths::ConfigPaths, themes::Palette};

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
    surface: ColorVariants,
    surface_variant: ColorVariants,
    on_background: ColorVariants,
    on_surface_variant: ColorVariants,
    primary: ColorVariants,
    secondary: ColorVariants,
    tertiary: ColorVariants,
    error: ColorVariants,
}

#[derive(Deserialize)]
struct ColorVariants {
    dark: String,
}

impl MatugenOutput {
    fn into_palette(self) -> Palette {
        let colors = self.colors;

        Palette {
            bg: colors.background.dark,
            surface: colors.surface.dark,
            elevated: colors.surface_variant.dark,
            fg: colors.on_background.dark,
            fg_muted: colors.on_surface_variant.dark,
            primary: colors.primary.dark.clone(),
            red: colors.error.dark,
            yellow: colors.tertiary.dark,
            green: colors.secondary.dark,
            blue: colors.primary.dark,
        }
    }
}
