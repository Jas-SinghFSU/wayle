/// Battery configuration
pub mod battery;
/// Clock configuration
pub mod clock;
/// Configuration error types
pub mod error;
/// General application configuration
pub mod general;
/// Configuration loading
pub mod loading;
/// Media configuration
pub mod media;
/// Module configuration
pub mod modules;
/// Configuration paths
pub mod paths;
/// Styling configuration
pub mod styling;

pub use error::Error;
use general::GeneralConfig;
use media::MediaConfig;
use modules::ModulesConfig;
use serde::{Deserialize, Serialize};

/// Main configuration structure for Wayle.
///
/// Represents the complete configuration schema that can be loaded
/// from TOML files. All fields have sensible defaults.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    /// General application settings.
    pub general: GeneralConfig,

    /// Module-specific configurations.
    pub modules: ModulesConfig,

    /// Media service configuration.
    pub media: MediaConfig,
}
