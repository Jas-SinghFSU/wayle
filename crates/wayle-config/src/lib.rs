//! Configuration management for Wayle.
//!
//! Handles schema definitions, configuration loading/saving, and file watching
//! for Wayle and its applets.

/// Documentation and metadata types for configuration schemas.
pub mod docs;

/// Configuration schema definitions.
pub mod schemas {
    /// Bar layout configuration
    pub mod bar;
    /// Battery module configuration
    pub mod battery;
    /// Clock module configuration
    pub mod clock;
    /// Media module configuration
    pub mod media;
    /// Module-specific configurations
    pub mod modules;
    /// Styling configuration
    pub mod styling;
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
    service::{ConfigService, ConfigServiceCli},
    watcher::FileWatcher,
};
use schemas::{bar::BarConfig, modules::ModulesConfig, styling::StylingConfig};
use wayle_derive::wayle_config;

/// Main configuration structure for Wayle.
///
/// Represents the complete configuration schema that can be loaded
/// from TOML files. All fields have sensible defaults.
#[wayle_config]
pub struct Config {
    /// Bar layout and module placement.
    pub bar: BarConfig,

    /// Styling configuration (theme, fonts, scale).
    pub styling: StylingConfig,

    /// Module-specific configurations.
    pub modules: ModulesConfig,
}
