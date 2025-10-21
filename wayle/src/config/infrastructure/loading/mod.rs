mod circular_detection;
mod file_creation;
mod merging;

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use circular_detection::CircularDetector;
use file_creation::create_default_config_file;
use merging::merge_toml_configs;
use toml::Value;

use super::error::Error;
use crate::config::Config;

impl Config {
    /// Loads a configuration file with support for importing other TOML files
    ///
    /// Import paths are specified using the `@` prefix in the TOML file.
    /// Imported configurations are merged with the main configuration,
    /// with the main configuration taking precedence in case of conflicts.
    /// Also checks for circular imports.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the main configuration file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The configuration file cannot be read
    /// - The TOML content is invalid
    /// - Any imported files cannot be loaded
    /// - The merged configuration is invalid
    /// - Circular imports are detected
    pub fn load_with_imports(path: &Path) -> Result<Config, Error> {
        if !path.exists() {
            create_default_config_file(path)?;
        }

        let canonical_path = path.canonicalize().map_err(|e| Error::IoError {
            path: path.to_path_buf(),
            details: format!("Failed to resolve path: {e}"),
        })?;

        let mut detector = CircularDetector::new();
        Self::load_config_with_tracking(&canonical_path, &mut detector)
    }

    /// Recursively collects all configuration files involved in imports.
    ///
    /// Starting from the given path, this method finds all imported files
    /// including transitive imports. Each file is listed only once even
    /// if imported multiple times.
    ///
    /// # Arguments
    /// * `path` - The root configuration file to start from
    ///
    /// # Returns
    /// A vector of all configuration file paths including the root file
    ///
    /// # Errors
    /// Returns error if any file cannot be read or contains invalid TOML
    pub fn get_all_config_files(path: &Path) -> Result<Vec<PathBuf>, Error> {
        let mut files = Vec::new();
        let mut visited = HashSet::new();

        Self::collect_config_files(path, &mut files, &mut visited)?;
        Ok(files)
    }

    fn load_config_with_tracking(
        path: &Path,
        detector: &mut CircularDetector,
    ) -> Result<Config, Error> {
        detector.detect_circular_import(path)?;
        detector.push_to_chain(path);

        let result = Self::load_main_config(path, detector);
        detector.pop_from_chain();
        result
    }

    fn load_main_config(path: &Path, detector: &mut CircularDetector) -> Result<Config, Error> {
        let main_config_content = fs::read_to_string(path)?;
        let import_paths = Self::extract_import_paths(&main_config_content)?;
        let imported_configs = Self::load_all_imports(path, &import_paths, detector)?;

        let main_config: Value =
            toml::from_str(&main_config_content).map_err(|e| Error::toml_parse(e, Some(path)))?;

        let merged_config = merge_toml_configs(imported_configs, main_config);
        merged_config
            .try_into()
            .map_err(|e| Error::ConfigValidation {
                component: String::from("config parsing"),
                details: format!("Configuration validation failed: {e}"),
            })
    }

    fn load_all_imports(
        base_path: &Path,
        import_paths: &[String],
        detector: &mut CircularDetector,
    ) -> Result<Vec<Value>, Error> {
        import_paths
            .iter()
            .map(|import_path| {
                let resolved_path = Self::resolve_import_path(base_path, import_path)?;
                let canonical_import = resolved_path
                    .canonicalize()
                    .map_err(|e| Error::import(e, &resolved_path))?;

                Self::load_imported_file_with_tracking(&canonical_import, detector)
            })
            .collect()
    }

    fn load_imported_file_with_tracking(
        path: &Path,
        detector: &mut CircularDetector,
    ) -> Result<Value, Error> {
        detector.detect_circular_import(path)?;
        detector.push_to_chain(path);

        let result = Self::load_toml_file_with_imports(path, detector);
        detector.pop_from_chain();
        result
    }

    fn load_toml_file_with_imports(
        path: &Path,
        detector: &mut CircularDetector,
    ) -> Result<Value, Error> {
        let content = fs::read_to_string(path).map_err(|e| Error::import(e, path))?;
        let import_paths = Self::extract_import_paths(&content)?;
        let imported_configs = Self::load_all_imports(path, &import_paths, detector)?;

        let main_value: Value =
            toml::from_str(&content).map_err(|e| Error::toml_parse(e, Some(path)))?;

        Ok(merge_toml_configs(imported_configs, main_value))
    }

    fn extract_import_paths(config_content: &str) -> Result<Vec<String>, Error> {
        let value = toml::from_str(config_content).map_err(|e| Error::toml_parse(e, None))?;

        let import_paths = if let Value::Table(table) = value {
            if let Some(Value::Array(imports)) = table.get("imports") {
                imports
                    .iter()
                    .filter_map(|v| v.as_str())
                    .filter(|s| s.starts_with('@'))
                    .map(|s| s.strip_prefix('@').unwrap_or(s).to_owned())
                    .collect::<Vec<String>>()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        Ok(import_paths)
    }

    fn resolve_import_path(base_path: &Path, import_path: &str) -> Result<PathBuf, Error> {
        let parent_dir = base_path.parent().ok_or_else(|| Error::ImportError {
            path: base_path.to_path_buf(),
            details: String::from("Invalid base path - no parent directory"),
        })?;

        let mut import_path_buf = PathBuf::from(import_path);
        if import_path_buf.extension().is_none() {
            import_path_buf.set_extension("toml");
        }

        let resolved_path = parent_dir.join(import_path_buf);
        Ok(resolved_path)
    }

    fn collect_config_files(
        path: &Path,
        files: &mut Vec<PathBuf>,
        visited: &mut HashSet<PathBuf>,
    ) -> Result<(), Error> {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

        if visited.contains(&canonical) {
            return Ok(());
        }

        visited.insert(canonical.clone());
        files.push(canonical.clone());

        if path.exists() {
            let content = fs::read_to_string(path)?;
            let import_paths = Self::extract_import_paths(&content)?;

            for import_path in import_paths {
                let resolved = Self::resolve_import_path(path, &import_path)?;
                Self::collect_config_files(&resolved, files, visited)?;
            }
        }

        Ok(())
    }
}
