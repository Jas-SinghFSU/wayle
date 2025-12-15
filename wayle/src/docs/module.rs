pub use crate::config::docs::{
    BehaviorConfigs, ModuleInfo, ModuleInfoProvider, SchemeFn, StylingConfigs,
};
use crate::config::schemas::clock::ClockConfig;

/// Retrieves information for all available Wayle modules.
///
/// Returns a comprehensive list of module metadata including their
/// configuration schemas for documentation generation.
///
/// # Returns
///
/// A vector containing `ModuleInfo` for each available module.
pub fn get_all_modules() -> Vec<ModuleInfo> {
    let clock_module = ClockConfig::module_info();

    vec![clock_module]
}
