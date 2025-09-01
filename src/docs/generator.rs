use std::{
    fs,
    path::{Path, PathBuf},
};

use thiserror::Error;

use super::{ModuleInfo, ModuleRegistry};
use crate::docs::generate_module_page;

/// Generates markdown documentation for Wayle modules.
///
/// Creates structured documentation files from module configuration
/// schemas, including behavior and styling options.
pub struct DocsGenerator {
    output_dir: String,
}

impl Default for DocsGenerator {
    fn default() -> Self {
        Self {
            output_dir: String::from("docs/config/modules"),
        }
    }
}

impl DocsGenerator {
    /// Creates a new documentation generator with default output directory.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a custom output directory for generated documentation.
    pub fn with_output_dir(mut self, output_dir: impl Into<String>) -> Self {
        self.output_dir = output_dir.into();
        self
    }

    /// Generates documentation for all registered modules.
    ///
    /// Creates markdown files for each module in the output directory,
    /// documenting their configuration options and schemas.
    ///
    /// # Errors
    ///
    /// Returns `DocsError::FileWrite` if direction creation failes.
    pub fn generate_all(&self) -> Result<(), DocsError> {
        fs::create_dir_all(&self.output_dir).map_err(|err| DocsError::FileWriteError {
            path: PathBuf::from(&self.output_dir),
            details: format!("Failed to create output directory: {err}"),
        })?;

        let modules = ModuleRegistry::get_all();

        for module in &modules {
            self.generate_single_module(module)?;
        }

        println!("Generated documentation for {} modules", modules.len());
        Ok(())
    }

    /// Generates documentation for a specific module by name.
    ///
    /// # Errors
    ///
    /// Returns `DocsError::InvalidModuleName` if the module doesn't exist.
    pub fn generate_module_by_name(&self, module_name: &str) -> Result<(), DocsError> {
        let module = ModuleRegistry::get_module_by_name(module_name).ok_or_else(|| {
            DocsError::ModuleNotFound {
                name: module_name.to_string(),
            }
        })?;

        self.generate_single_module(&module)
    }

    /// Returns a list of all available module names.
    pub fn list_modules(&self) -> Vec<String> {
        ModuleRegistry::list_module_names()
    }

    fn generate_single_module(&self, module: &ModuleInfo) -> Result<(), DocsError> {
        let content = generate_module_page(module)?;
        let filename = format!("{}.md", module.name);
        let filepath = Path::new(&self.output_dir).join(filename);

        fs::write(&filepath, content).map_err(|err| DocsError::FileWriteError {
            path: filepath.clone(),
            details: err.to_string(),
        })?;

        println!("Generated {}", filepath.display());
        Ok(())
    }
}

/// Errors that can occur during documentation generation.
#[derive(Error, Debug)]
pub enum DocsError {
    /// Failed to write documentation file
    #[error("failed to write documentation to '{path}': {details}")]
    FileWriteError {
        /// Path where write failed
        path: PathBuf,
        /// Write error details
        details: String,
    },

    /// Invalid module name provided
    #[error("invalid module name '{name}': {reason}")]
    InvalidModuleName {
        /// The invalid module name
        name: String,
        /// Reason why the name is invalid
        reason: String,
    },

    /// Schema conversion failed
    #[error("failed to convert schema for '{module}': {details}")]
    SchemaConversionError {
        /// Module whose schema failed conversion
        module: String,
        /// Conversion error details
        details: String,
    },

    /// Module not found in registry
    #[error("module '{name}' not found in registry")]
    ModuleNotFound {
        /// The module name that wasn't found
        name: String,
    },
}
