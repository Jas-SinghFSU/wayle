use std::fs;

use serde::Deserialize;
use wayle_config::infrastructure::{paths::ConfigPaths, themes::Palette};

use super::color;
use crate::Error;

pub(crate) struct MatugenProvider;

impl MatugenProvider {
    pub(crate) fn load(is_light: bool) -> Result<Palette, Error> {
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

        Ok(output.into_palette(is_light))
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
    light: ColorValue,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ColorValue {
    Plain(String),
    Nested { color: String },
}

impl ColorVariants {
    fn pick(self, is_light: bool) -> String {
        if is_light {
            self.light.as_color()
        } else {
            self.dark.as_color()
        }
    }
}

impl MatugenOutput {
    fn into_palette(self, is_light: bool) -> Palette {
        let colors = self.colors;
        let primary = colors.primary.pick(is_light);
        let bg = colors.background.pick(is_light);
        let layers = color::derive_layers(&bg, is_light);

        Palette {
            bg: layers.bg,
            surface: layers.surface,
            elevated: layers.elevated,
            fg: colors.on_background.pick(is_light),
            fg_muted: colors.on_surface_variant.pick(is_light),
            primary: primary.clone(),
            red: colors.error.pick(is_light),
            yellow: colors.tertiary.pick(is_light),
            green: colors.secondary.pick(is_light),
            blue: primary,
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
            "background": { "dark": "#101112", "light": "#fafbfc" },
            "surface": { "dark": "#202122", "light": "#eaebec" },
            "surface_variant": { "dark": "#303132", "light": "#dadbdc" },
            "on_background": { "dark": "#f0f1f2", "light": "#101112" },
            "on_surface_variant": { "dark": "#a0a1a2", "light": "#505152" },
            "primary": { "dark": "#4090ff", "light": "#2060cc" },
            "secondary": { "dark": "#40ff90", "light": "#20cc60" },
            "tertiary": { "dark": "#ffcf40", "light": "#cc9f20" },
            "error": { "dark": "#ff4040", "light": "#cc2020" }
        }
    }"##;

    const NEW_JSON: &str = r##"{
        "colors": {
            "background": { "dark": { "color": "#101112" }, "light": { "color": "#fafbfc" } },
            "surface": { "dark": { "color": "#202122" }, "light": { "color": "#eaebec" } },
            "surface_variant": { "dark": { "color": "#303132" }, "light": { "color": "#dadbdc" } },
            "on_background": { "dark": { "color": "#f0f1f2" }, "light": { "color": "#101112" } },
            "on_surface_variant": { "dark": { "color": "#a0a1a2" }, "light": { "color": "#505152" } },
            "primary": { "dark": { "color": "#4090ff" }, "light": { "color": "#2060cc" } },
            "secondary": { "dark": { "color": "#40ff90" }, "light": { "color": "#20cc60" } },
            "tertiary": { "dark": { "color": "#ffcf40" }, "light": { "color": "#cc9f20" } },
            "error": { "dark": { "color": "#ff4040" }, "light": { "color": "#cc2020" } }
        }
    }"##;

    #[test]
    fn parses_old_matugen_shape() {
        let output: MatugenOutput = serde_json::from_str(OLD_JSON).unwrap();
        let palette = output.into_palette(false);
        assert_eq!(palette.surface, "#101112");
        assert_eq!(palette.primary, "#4090ff");
    }

    #[test]
    fn parses_new_matugen_shape() {
        let output: MatugenOutput = serde_json::from_str(NEW_JSON).unwrap();
        let palette = output.into_palette(false);
        assert_eq!(palette.surface, "#101112");
        assert_eq!(palette.primary, "#4090ff");
    }

    #[test]
    fn parses_light_mode() {
        let output: MatugenOutput = serde_json::from_str(OLD_JSON).unwrap();
        let palette = output.into_palette(true);
        assert_eq!(palette.primary, "#2060cc");
        assert_eq!(palette.fg, "#101112");
        assert_eq!(palette.green, "#20cc60");
    }
}
