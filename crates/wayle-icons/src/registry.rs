//! GTK IconTheme integration for Wayle icons.
//!
//! The registry ensures GTK can discover icons in the Wayle icon directory.

use std::{fs, path::PathBuf};

use gtk4::gdk;
use tracing::info;

use crate::error::{Error, Result};

const INDEX_THEME_CONTENT: &str = r#"[Icon Theme]
Name=Wayle Icons
Comment=Icons installed by Wayle
Directories=hicolor/scalable/actions

[hicolor/scalable/actions]
Size=48
MinSize=16
MaxSize=512
Type=Scalable
"#;

/// Manages GTK IconTheme registration for Wayle icons.
///
/// Call [`IconRegistry::init`] at application startup to ensure
/// GTK can discover icons installed via `wayle icons install`.
#[derive(Debug, Clone)]
pub struct IconRegistry {
    base_path: PathBuf,
}

impl IconRegistry {
    /// Creates a new registry with the default icon directory.
    ///
    /// The default path is `~/.local/share/wayle/icons/`.
    ///
    /// # Errors
    ///
    /// Returns error if `$HOME` is not set and `$XDG_DATA_HOME` is also unset.
    pub fn new() -> Result<Self> {
        Ok(Self {
            base_path: Self::default_path()?,
        })
    }

    /// Creates a registry with a custom icon directory.
    ///
    /// Useful for testing or custom configurations.
    pub fn with_path(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    /// Returns the default icon directory path.
    ///
    /// Uses `$XDG_DATA_HOME/wayle/icons` or `~/.local/share/wayle/icons`.
    ///
    /// # Errors
    ///
    /// Returns error if `$HOME` is not set and `$XDG_DATA_HOME` is also unset.
    pub fn default_path() -> Result<PathBuf> {
        let data_home = match std::env::var("XDG_DATA_HOME") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                let home = std::env::var("HOME").map_err(|_| Error::HomeNotSet)?;
                PathBuf::from(home).join(".local").join("share")
            }
        };

        Ok(data_home.join("wayle").join("icons"))
    }

    /// Returns the base path for this registry.
    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    /// Returns the directory where SVG icons are stored.
    ///
    /// This is `<base_path>/hicolor/scalable/actions/`.
    pub fn icons_dir(&self) -> PathBuf {
        self.base_path
            .join("hicolor")
            .join("scalable")
            .join("actions")
    }

    /// Ensures the icon directory structure and index.theme exist.
    ///
    /// Use this in CLI contexts where GTK is not available.
    /// For GUI applications, use [`init`] instead.
    ///
    /// # Errors
    ///
    /// Returns error if directory creation or file writing fails.
    pub fn ensure_setup(&self) -> Result<()> {
        self.ensure_directory_structure()?;
        self.ensure_index_theme()?;
        Ok(())
    }

    /// Initializes the icon registry with GTK.
    ///
    /// This method:
    /// 1. Creates the icon directory structure if it doesn't exist
    /// 2. Creates the `index.theme` file if missing
    /// 3. Registers the directory with GTK's IconTheme
    ///
    /// Call this once at application startup before displaying any widgets
    /// that use Wayle icons.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Directory creation fails
    /// - File writing fails
    /// - No display is available (GTK not initialized)
    pub fn init(&self) -> Result<()> {
        self.ensure_setup()?;
        self.register_with_gtk()?;

        info!(path = %self.base_path.display(), "Icon registry initialized");
        Ok(())
    }

    /// Ensures the icon directory structure exists.
    fn ensure_directory_structure(&self) -> Result<()> {
        let icons_dir = self.icons_dir();

        if !icons_dir.exists() {
            fs::create_dir_all(&icons_dir).map_err(|err| Error::DirectoryError {
                path: icons_dir,
                details: err.to_string(),
            })?;
        }

        Ok(())
    }

    /// Ensures the index.theme file exists.
    fn ensure_index_theme(&self) -> Result<()> {
        let index_path = self.base_path.join("index.theme");

        if !index_path.exists() {
            fs::write(&index_path, INDEX_THEME_CONTENT).map_err(|err| Error::WriteError {
                path: index_path,
                details: err.to_string(),
            })?;
        }

        Ok(())
    }

    /// Registers the icon directory with GTK's IconTheme.
    fn register_with_gtk(&self) -> Result<()> {
        let display = gdk::Display::default()
            .ok_or_else(|| Error::RegistryError("no display".to_string()))?;

        let icon_theme = gtk4::IconTheme::for_display(&display);
        icon_theme.add_search_path(&self.base_path);

        Ok(())
    }

    /// Checks if the icon directory and theme are properly set up.
    ///
    /// Returns `true` if:
    /// - The icons directory exists
    /// - The index.theme file exists
    pub fn is_valid(&self) -> bool {
        self.icons_dir().exists() && self.base_path.join("index.theme").exists()
    }
}
