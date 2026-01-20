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
    #[allow(dead_code)]
    color0: String,
    color1: String,
    color2: String,
    color3: String,
    color4: String,
    color7: String,
    color8: String,
}

fn darken_hex(hex: &str, factor: f32) -> String {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);

    let r = (f32::from(r) * factor).round() as u8;
    let g = (f32::from(g) * factor).round() as u8;
    let b = (f32::from(b) * factor).round() as u8;

    format!("#{r:02X}{g:02X}{b:02X}")
}

impl WallustOutput {
    fn into_palette(self) -> Palette {
        let bg = darken_hex(&self.background, 0.6);

        Palette {
            bg,
            surface: self.background,
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
