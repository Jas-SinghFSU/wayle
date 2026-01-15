use std::{fs, path::Path};

use tracing::{error, info};

use crate::{
    Config, Error,
    infrastructure::themes::{Palette, palettes::builtins},
    schemas::styling::ThemeEntry,
};

/// Loads and mounts the built in themes and themes in the ~/.config/wayle/themes
/// directory into config.style.theme.available
pub fn load_themes(config: &Config, themes_dir: &Path) {
    let mut all_themes: Vec<ThemeEntry> = builtins()
        .into_iter()
        .map(|palette| ThemeEntry {
            palette,
            builtin: true,
        })
        .collect();

    let Ok(entries) = fs::read_dir(themes_dir) else {
        info!("Themes directory not found...");
        return;
    };

    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };

        let path = entry.path();

        let Ok(theme) = get_theme_from_file(&path) else {
            error!(path = %path.display(), "cannot load theme");
            continue;
        };

        if all_themes
            .iter()
            .any(|t| t.palette.name == theme.palette.name)
        {
            error!(theme = %theme.palette.name, "Theme already exists, skipping");
            continue;
        }

        all_themes.push(theme);
    }

    config.styling.theme.available.set(all_themes);
}

fn get_theme_from_file(path: &Path) -> Result<ThemeEntry, Error> {
    if path.extension().is_none_or(|ext| ext != "toml") {
        return Err(Error::ThemeNotToml { path: path.into() });
    }

    let content = fs::read_to_string(path).map_err(|source| Error::ThemeRead {
        path: path.into(),
        source,
    })?;

    let palette: Palette = toml::from_str(&content).map_err(|source| Error::ThemeParse {
        path: path.into(),
        source,
    })?;

    Ok(ThemeEntry {
        palette,
        builtin: false,
    })
}
