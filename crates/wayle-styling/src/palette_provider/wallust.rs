//! Wallust palette provider.

use std::fs;

use serde::Deserialize;
use wayle_config::infrastructure::{paths::ConfigPaths, themes::Palette};

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
    color0: String,
    color1: String,
    color2: String,
    color3: String,
    color4: String,
    color7: String,
    color8: String,
}

impl WallustOutput {
    fn into_palette(self) -> Palette {
        Palette {
            bg: self.background,
            surface: self.color0,
            elevated: self.color8,
            fg: self.foreground,
            fg_muted: self.color7,
            primary: self.color4.clone(),
            red: self.color1,
            yellow: self.color3,
            green: self.color2,
            blue: self.color4,
        }
    }
}
