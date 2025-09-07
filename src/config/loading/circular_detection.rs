use std::path::{Path, PathBuf};

use crate::config::error::WayleError;

/// Tracks import chains for circular detection
pub(crate) struct CircularDetector {
    /// Current import chain (for circular detection)
    import_chain: Vec<PathBuf>,
}

impl CircularDetector {
    pub fn new() -> Self {
        Self {
            import_chain: Vec::new(),
        }
    }

    /// Checks if a file can be visited without creating a cycle
    /// Returns an error if a circular import is detected
    pub fn detect_circular_import(&self, path: &Path) -> Result<(), WayleError> {
        if self.import_chain.contains(&path.to_path_buf()) {
            let chain_display: Vec<String> = self
                .import_chain
                .iter()
                .map(|p| {
                    p.file_name()
                        .unwrap_or(p.as_os_str())
                        .to_string_lossy()
                        .to_string()
                })
                .collect();

            let current_file = path
                .file_name()
                .unwrap_or(path.as_os_str())
                .to_string_lossy();

            return Err(WayleError::ConfigValidation {
                component: String::from("import system"),
                details: format!(
                    "Circular import detected: {} -> {}",
                    chain_display.join(" -> "),
                    current_file
                ),
            });
        }
        Ok(())
    }

    /// Adds a file to the import chain for tracking
    pub fn push_to_chain(&mut self, path: &Path) {
        self.import_chain.push(path.to_path_buf());
    }

    /// Removes a file from the import chain when done processing
    pub fn pop_from_chain(&mut self) {
        self.import_chain.pop();
    }
}
