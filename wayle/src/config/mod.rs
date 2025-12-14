/// Configuration schema definitions
pub mod schemas {
    /// Battery configuration
    pub mod battery;
    /// Clock configuration
    pub mod clock;
    /// General application configuration
    pub mod general;
    /// Media configuration
    pub mod media;
    /// Module configuration
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
    /// TOML path utilities
    pub mod toml_path;
    /// File watching
    pub mod watcher;
}

pub use infrastructure::error::Error;
pub use infrastructure::persistence::PersistenceWatcher;
pub use infrastructure::service::{ConfigService, ConfigServiceCli};
pub use infrastructure::watcher::FileWatcher;

use schemas::general::GeneralConfig;
use schemas::media::MediaConfig;
use schemas::modules::ModulesConfig;
use serde::{Deserialize, Serialize};
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

/// Main configuration structure for Wayle.
///
/// Represents the complete configuration schema that can be loaded
/// from TOML files. All fields have sensible defaults.
#[derive(Debug, Clone, Serialize, Deserialize, Default, ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges)]
#[serde(default)]
pub struct Config {
    /// General application settings.
    pub general: GeneralConfig,

    /// Module-specific configurations.
    pub modules: ModulesConfig,

    /// Media service configuration.
    pub media: MediaConfig,
}
