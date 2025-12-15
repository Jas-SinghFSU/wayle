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

/// Trait for types that can provide module information for documentation.
///
/// Implement this trait on module configuration structs to define their
/// metadata, behavior schemas, and styling schemas in a centralized way.
pub trait ModuleInfoProvider {
    /// Returns the module information including metadata and schemas.
    fn module_info() -> ModuleInfo;
}

/// Represents metadata and configuration schemas for a Wayle module.
///
/// Contains all the information needed to document and configure a module,
/// including its behavior schema and associated styling component schemas.
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
