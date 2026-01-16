//! Documentation and metadata types for configuration schemas.

use schemars::Schema;

/// Function type that generates a JSON schema.
pub type SchemeFn = fn() -> Schema;

/// Collection of styling configuration schemas for a module.
///
/// Maps styling component names to their schema generator functions.
pub type StylingConfigs = Vec<(String, SchemeFn)>;

/// Collection of behavior configuration schemas for a module.
///
/// Maps behavior component names to their schema generator functions.
pub type BehaviorConfigs = Vec<(String, SchemeFn)>;

/// Module documentation provider.
///
/// Module configuration structs implement this to expose their metadata,
/// behavior schemas, and styling schemas for documentation generation.
pub trait ModuleInfoProvider {
    /// Returns the module information including metadata and schemas.
    fn module_info() -> ModuleInfo;
}

/// Metadata and configuration schemas for a Wayle module.
///
/// Aggregates display name, description, behavior schemas, and styling
/// component schemas for documentation generation.
pub struct ModuleInfo {
    /// The display name of the module (e.g., "Clock", "Battery").
    pub name: String,
    /// Unicode icon or emoji representing the module visually.
    pub icon: String,
    /// Human-readable description of the module's purpose and functionality.
    pub description: String,
    /// Map of behavior component names to their schema generator functions.
    pub behavior_configs: BehaviorConfigs,
    /// Map of styling component names to their schema generator functions.
    pub styling_configs: StylingConfigs,
}
