//! Wallust palette provider.

use std::fs;

use serde::Deserialize;
use wayle_config::infrastructure::{paths::ConfigPaths, themes::Palette};

use super::color;
use crate::Error;

pub(crate) struct WallustProvider;

impl WallustProvider {
    pub(crate) fn load(is_light: bool) -> Result<Palette, Error> {
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

        Ok(output.into_palette(is_light))
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
    fn into_palette(self, is_light: bool) -> Palette {
        let layers = color::derive_layers(&self.background, is_light);

        Palette {
            bg: layers.bg,
            surface: layers.surface,
            elevated: layers.elevated,
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
