//! Configuration schema definitions and validation.
//!
//! Defines the complete configuration structure for Wayle, including
//! general settings and module-specific configurations. All configurations
//! are serializable to/from TOML format.

mod battery;
mod clock;
mod error;
mod general;
mod loading;
mod media;
mod modules;
mod paths;
mod styling;

#[cfg(test)]
mod tests;

pub use clock::ClockConfig;
pub use error::{Result, WayleError};
use general::GeneralConfig;
pub use media::MediaConfig;
use modules::ModulesConfig;
pub use paths::ConfigPaths;
use serde::{Deserialize, Serialize};
pub use styling::*;

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
