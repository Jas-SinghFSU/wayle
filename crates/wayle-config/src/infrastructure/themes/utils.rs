use std::{fs, path::Path};

use tracing::error;

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
        error!("Failed to read themes directory");
        return;
    };

    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };

        let path = entry.path();

        let Ok(theme) = get_theme_from_file(&path) else {
            error!("Failed to load theme at {}", path.display());
            continue;
        };

        if all_themes
            .iter()
            .any(|t| t.palette.name == theme.palette.name)
        {
            error!("Theme '{}' already exists, skipping", theme.palette.name);
            continue;
        }

        all_themes.push(theme);
    }

    config.styling.theme.available.set(all_themes);
}

fn get_theme_from_file(path: &Path) -> Result<ThemeEntry, Error> {
    if path.extension().is_none_or(|ext| ext != "toml") {
        return Err(Error::ThemeSerializationError {
            path: path.into(),
            details: String::from("File is not a toml file"),
        });
    }

    let content = fs::read_to_string(path).map_err(|e| Error::ThemeSerializationError {
        path: path.into(),
        details: format!("Failed to read file: {e}"),
    })?;

    let palette: Palette =
        toml::from_str(&content).map_err(|e| Error::ThemeSerializationError {
            path: path.into(),
            details: format!("Failed to parse TOML: {e}"),
        })?;

    Ok(ThemeEntry {
        palette,
        builtin: false,
    })
}
