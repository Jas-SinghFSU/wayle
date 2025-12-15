use std::path::{Path, PathBuf};

use crate::infrastructure::error::Error;

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
    pub fn detect_circular_import(&self, path: &Path) -> Result<(), Error> {
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

            return Err(Error::ConfigValidation {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_detector() {
        let detector = CircularDetector::new();
        assert!(detector.import_chain.is_empty());
    }

    #[test]
    fn allows_first_visit_to_file() {
        let detector = CircularDetector::new();
        let path = Path::new("/config/base.toml");

        assert!(detector.detect_circular_import(path).is_ok());
    }

    #[test]
    fn detects_circular_import() {
        let mut detector = CircularDetector::new();
        let path = PathBuf::from("/config/base.toml");

        detector.push_to_chain(&path);
        let result = detector.detect_circular_import(&path);

        assert!(result.is_err());
    }

    #[test]
    fn allows_revisit_after_pop() {
        let mut detector = CircularDetector::new();
        let path = PathBuf::from("/config/base.toml");

        detector.push_to_chain(&path);
        detector.pop_from_chain();
        let result = detector.detect_circular_import(&path);

        assert!(result.is_ok());
    }

    #[test]
    fn tracks_chain_correctly() {
        let mut detector = CircularDetector::new();
        let path_a = PathBuf::from("/config/a.toml");
        let path_b = PathBuf::from("/config/b.toml");
        let path_c = PathBuf::from("/config/c.toml");

        detector.push_to_chain(&path_a);
        detector.push_to_chain(&path_b);

        assert!(detector.detect_circular_import(&path_c).is_ok());
        assert!(detector.detect_circular_import(&path_a).is_err());
        assert!(detector.detect_circular_import(&path_b).is_err());
    }

    #[test]
    fn error_message_contains_chain() {
        let mut detector = CircularDetector::new();
        let path_a = PathBuf::from("/config/a.toml");
        let path_b = PathBuf::from("/config/b.toml");

        detector.push_to_chain(&path_a);
        detector.push_to_chain(&path_b);

        let result = detector.detect_circular_import(&path_a);
        let error_message = format!("{:?}", result.unwrap_err());

        assert!(error_message.contains("a.toml"));
        assert!(error_message.contains("b.toml"));
    }
}
