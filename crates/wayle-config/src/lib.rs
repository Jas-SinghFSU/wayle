//! Configuration management for Wayle.
//!
//! Handles schema definitions, configuration loading/saving, and file watching
//! for Wayle and its applets.

/// Documentation and metadata types for configuration schemas.
pub mod docs;

/// Configuration schema definitions.
pub mod schemas {
    /// Bar layout configuration.
    pub mod bar;
    /// General Wayle configuration.
    pub mod general;
    /// Module-specific configurations.
    pub mod modules;
    /// Styling configuration.
    pub mod styling;
    /// Wallpaper service configuration.
    pub mod wallpaper;
}

/// Configuration infrastructure
pub mod infrastructure {
    /// Configuration error types
    pub mod error;
    /// Configuration loading
    pub mod loading;
    /// Configuration paths
    pub mod paths;
    /// Configuration persistence
    pub mod persistence;
    /// JSON Schema generation for editor support
    pub mod schema;
    /// Secret resolution from environment variables
    pub mod secrets;
    /// Configuration service
    pub mod service;
    /// Wayle theme management and discovery
    pub mod themes;
    /// TOML path utilities
    pub mod toml_path;
    /// File watching
    pub mod watcher;
}

pub use infrastructure::{
    error::Error,
    paths::ConfigPaths,
    persistence::PersistenceWatcher,
    schema::generate_schema,
    secrets,
    service::{ConfigService, ConfigServiceCli},
    watcher::FileWatcher,
};
use schemas::{
    bar::BarConfig, modules::ModulesConfig, styling::StylingConfig, wallpaper::WallpaperConfig,
};
use wayle_derive::wayle_config;

use crate::schemas::general::GeneralConfig;

/// Main configuration structure for Wayle.
///
/// Represents the complete configuration schema that can be loaded
/// from TOML files. All fields have sensible defaults.
#[wayle_config]
pub struct Config {
    /// TOML files to import and merge before this config.
    ///
    /// Paths are relative to the config file and prefixed with `@`.
    /// Imported values are overridden by values in this file.
    ///
    /// ```toml
    /// imports = ["@themes.toml", "@modules/clock.toml"]
    /// ```
    #[wayle(skip)]
    #[serde(default)]
    pub imports: Vec<String>,

    /// General Wayle settings.
    pub general: GeneralConfig,

    /// Bar layout and module placement.
    pub bar: BarConfig,

    /// Styling configuration (theme, fonts, scale).
    pub styling: StylingConfig,

    /// Module-specific configurations.
    pub modules: ModulesConfig,

    /// Wallpaper service settings.
    pub wallpaper: WallpaperConfig,
}
