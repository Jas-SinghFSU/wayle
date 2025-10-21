use crate::{
    config::schemas::clock::ClockConfig,
    docs::module::{ModuleInfo, ModuleInfoProvider},
};

/// Central registry for all available modules in the Wayle system.
///
/// Provides methods to discover, list, and retrieve information about
/// registered modules including their configuration schemas and metadata.
pub struct ModuleRegistry;

impl ModuleRegistry {
    /// Returns information about all registered modules.
    ///
    /// Collects and returns module metadata including names, descriptions,
    /// icons, and configuration schemas for every module in the system.
    pub fn get_all() -> Vec<ModuleInfo> {
        let module_info = Self::get_module_info::<ClockConfig>();
        let modules: Vec<ModuleInfo> = vec![module_info];
        modules
    }

    /// Retrieves module information by its name.
    ///
    /// Searches for a module with the specified name and returns its
    /// metadata if found.
    pub fn get_module_by_name(name: &str) -> Option<ModuleInfo> {
        Self::get_all()
            .into_iter()
            .find(|module| module.name == name)
    }

    /// Returns a list of all registered module names.
    ///
    /// Useful for discovery and validation of available modules
    /// without retrieving full module information.
    pub fn list_module_names() -> Vec<String> {
        Self::get_all()
            .into_iter()
            .map(|module| module.name)
            .collect()
    }

    fn get_module_info<T: ModuleInfoProvider>() -> ModuleInfo {
        T::module_info()
    }
}
