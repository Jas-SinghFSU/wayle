use std::fs;

use serde::Deserialize;
use wayle_config::infrastructure::{paths::ConfigPaths, themes::Palette};

use crate::{Error, palette_provider::PaletteProvider};

pub(crate) struct PyWallustProvider;

impl PaletteProvider for PyWallustProvider {
    fn load() -> Result<Palette, Error> {
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

        Ok(output.into_palette())
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
    color0: String,
    color1: String,
    color2: String,
    color3: String,
    color4: String,
    color7: String,
    color8: String,
}

impl PywalOutput {
    fn into_palette(self) -> Palette {
        Palette {
            bg: self.special.background,
            surface: self.colors.color0,
            elevated: self.colors.color8,
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
