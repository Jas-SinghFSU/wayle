//! Pywal palette provider.

use std::fs;

use serde::Deserialize;
use wayle_config::infrastructure::{paths::ConfigPaths, themes::Palette};

use super::color;
use crate::Error;

pub(crate) struct PywalProvider;

impl PywalProvider {
    pub(crate) fn load(is_light: bool) -> Result<Palette, Error> {
        let path = ConfigPaths::pywal_colors().map_err(|_| {
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "cannot determine pywal colors path",
            ))
        })?;

        if !path.exists() {
            return Err(Error::PaletteNotFound(path));
        }

        let content = fs::read_to_string(&path)?;
        let output: PywalOutput = serde_json::from_str(&content)?;

        Ok(output.into_palette(is_light))
    }
}

#[derive(Deserialize)]
struct PywalOutput {
    special: SpecialColors,
    colors: TerminalColors,
}

#[derive(Deserialize)]
struct SpecialColors {
    background: String,
    foreground: String,
}

#[derive(Deserialize)]
struct TerminalColors {
    color1: String,
    color2: String,
    color3: String,
    color4: String,
    color7: String,
}

impl PywalOutput {
    fn into_palette(self, is_light: bool) -> Palette {
        let layers = color::derive_layers(&self.special.background, is_light);

        Palette {
            bg: layers.bg,
            surface: layers.surface,
            elevated: layers.elevated,
            fg: self.special.foreground,
            fg_muted: self.colors.color7,
            primary: self.colors.color4.clone(),
            red: self.colors.color1,
            yellow: self.colors.color3,
            green: self.colors.color2,
            blue: self.colors.color4,
        }
    }
}
