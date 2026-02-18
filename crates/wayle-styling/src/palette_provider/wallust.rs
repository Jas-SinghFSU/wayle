//! Wallust palette provider.

use std::fs;

use serde::Deserialize;
use wayle_config::infrastructure::{paths::ConfigPaths, themes::Palette};

use super::color;
use crate::{Error, palette_provider::PaletteProvider};

pub(crate) struct WallustProvider;

impl PaletteProvider for WallustProvider {
    fn load() -> Result<Palette, Error> {
        let path = ConfigPaths::wallust_colors().map_err(|_| {
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "cannot determine wallust colors path",
            ))
        })?;

        if !path.exists() {
            return Err(Error::PaletteNotFound(path));
        }

        let content = fs::read_to_string(&path)?;
        let output: WallustOutput = serde_json::from_str(&content)?;

        Ok(output.into_palette())
    }
}

#[derive(Deserialize)]
struct WallustOutput {
    background: String,
    foreground: String,
    color3: String,
    color4: String,
    color5: String,
    color6: String,
    color7: String,
}

impl WallustOutput {
    fn into_palette(self) -> Palette {
        let bg = &self.background;

        Palette {
            bg: bg.clone(),
            surface: color::lighten(bg, 0.03),
            elevated: color::lighten(bg, 0.06),
            fg: self.foreground,
            fg_muted: self.color7,
            primary: self.color6.clone(),
            red: self.color3,
            yellow: self.color5,
            green: self.color4,
            blue: self.color6,
        }
    }
}
