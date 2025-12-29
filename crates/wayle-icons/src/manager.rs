//! Icon installation and removal operations.
//!
//! The manager handles fetching icons from CDN sources and storing them
//! in the icon directory for GTK to discover.

use std::{fs, path::Path};

use tracing::{debug, info, warn};

use crate::{
    error::{Error, Result},
    registry::IconRegistry,
    sources::IconSource,
    transform,
};

/// Manages icon installation and removal.
///
/// Uses [`IconRegistry`] to determine where icons are stored and provides
/// async methods to fetch icons from CDN sources.
#[derive(Debug, Clone)]
pub struct IconManager {
    registry: IconRegistry,
    client: reqwest::Client,
}

impl IconManager {
    /// Creates a new manager with the default icon directory.
    ///
    /// # Errors
    ///
    /// Returns error if `$HOME` is not set and `$XDG_DATA_HOME` is also unset.
    pub fn new() -> Result<Self> {
        Ok(Self {
            registry: IconRegistry::new()?,
            client: reqwest::Client::new(),
        })
    }

    /// Creates a manager with a custom registry.
    ///
    /// Useful for testing or custom configurations.
    pub fn with_registry(registry: IconRegistry) -> Self {
        Self {
            registry,
            client: reqwest::Client::new(),
        }
    }

    /// Returns the registry used by this manager.
    pub fn registry(&self) -> &IconRegistry {
        &self.registry
    }

    /// Installs icons from a source by fetching from CDN.
    ///
    /// # Arguments
    ///
    /// * `source` - The icon source (Tabler, SimpleIcons, etc.)
    /// * `slugs` - Icon identifiers to install (e.g., "home", "settings")
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - HTTP request fails
    /// - Response is not valid SVG
    /// - File write fails
    pub async fn install(&self, source: &dyn IconSource, slugs: &[&str]) -> Result<Vec<String>> {
        let icons_dir = self.registry.icons_dir();
        fs::create_dir_all(&icons_dir).map_err(|err| Error::DirectoryError {
            path: icons_dir.clone(),
            details: err.to_string(),
        })?;

        let mut installed = Vec::with_capacity(slugs.len());

        for slug in slugs {
            match self.install_single(source, slug, &icons_dir).await {
                Ok(name) => {
                    info!(icon = %name, source = source.cli_name(), "Installed icon");
                    installed.push(name);
                }
                Err(err) => {
                    warn!(slug = %slug, source = source.cli_name(), error = %err, "Failed to install icon");
                    return Err(err);
                }
            }
        }

        Ok(installed)
    }

    /// Installs a single icon from a source.
    async fn install_single(
        &self,
        source: &dyn IconSource,
        slug: &str,
        icons_dir: &Path,
    ) -> Result<String> {
        let url = source.cdn_url(slug);
        let icon_name = source.icon_name(slug);

        debug!(url = %url, "Fetching icon");

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(Error::FetchError {
                slug: slug.to_string(),
                icon_source: source.cli_name().to_string(),
                details: format!("HTTP {}", response.status()),
            });
        }

        let svg_content = response.text().await?;

        Self::validate_svg(&svg_content, slug)?;

        let transformed = transform::to_symbolic(&svg_content);

        let file_path = icons_dir.join(format!("{icon_name}-symbolic.svg"));
        fs::write(&file_path, &transformed).map_err(|err| Error::WriteError {
            path: file_path,
            details: err.to_string(),
        })?;

        Ok(format!("{icon_name}-symbolic"))
    }

    /// Removes an installed icon by name.
    ///
    /// # Arguments
    ///
    /// * `icon_name` - Full icon name including prefix (e.g., "tb-home")
    ///
    /// # Errors
    ///
    /// Returns error if icon doesn't exist or deletion fails.
    pub fn remove(&self, icon_name: &str) -> Result<()> {
        let file_path = self.registry.icons_dir().join(format!("{icon_name}.svg"));

        if !file_path.exists() {
            return Err(Error::NotFound {
                name: icon_name.to_string(),
            });
        }

        fs::remove_file(&file_path).map_err(|err| Error::DeleteError {
            name: icon_name.to_string(),
            details: err.to_string(),
        })?;

        info!(icon = %icon_name, "Removed icon");
        Ok(())
    }

    /// Lists all installed icons.
    ///
    /// Returns icon names without the `.svg` extension.
    pub fn list(&self) -> Vec<String> {
        let icons_dir = self.registry.icons_dir();

        let Ok(entries) = fs::read_dir(&icons_dir) else {
            return Vec::new();
        };

        entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();

                if path.extension().is_some_and(|ext| ext == "svg") {
                    path.file_stem().and_then(|s| s.to_str()).map(String::from)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Checks if an icon is installed.
    ///
    /// # Arguments
    ///
    /// * `icon_name` - Full icon name including prefix (e.g., "tb-home")
    pub fn is_installed(&self, icon_name: &str) -> bool {
        self.registry
            .icons_dir()
            .join(format!("{icon_name}.svg"))
            .exists()
    }

    /// Validates that content is valid SVG.
    fn validate_svg(content: &str, slug: &str) -> Result<()> {
        let trimmed = content.trim();

        if !trimmed.starts_with('<') {
            return Err(Error::InvalidSvg {
                slug: slug.to_string(),
                details: "content does not start with '<'".to_string(),
            });
        }

        if !trimmed.contains("<svg") {
            return Err(Error::InvalidSvg {
                slug: slug.to_string(),
                details: "missing <svg> element".to_string(),
            });
        }

        Ok(())
    }
}
