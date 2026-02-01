//! Secret value resolution from environment variables.
//!
//! Loads `.env` and `.*.env` files from the config directory and resolves
//! `$VAR_NAME` references in config values. Files are watched for
//! changes and automatically reloaded at runtime.
//!
//! # Usage
//!
//! Create `~/.config/wayle/.env` (or `.secrets.env`, `.api.env`, etc.):
//!
//! ```text
//! WEATHER_API_KEY=your-api-key-here
//! ```
//!
//! Then reference it in `config.toml`:
//!
//! ```toml
//! [modules.weather]
//! provider = "visual-crossing"
//! visual-crossing-key = "$WEATHER_API_KEY"
//! ```
//!
//! When the `.env` file changes, the value is automatically re-resolved.

use std::path::{Path, PathBuf};

use glob::glob;
use tracing::{debug, info, warn};

/// Loads `.env` and `.*.env` files from the given directory into the environment.
///
/// Files are loaded in alphabetical order. Later files override earlier ones.
pub fn load_env_files(config_dir: &Path) {
    load_env_files_inner(config_dir, false);
}

/// Reloads `.env` and `.*.env` files from the given directory.
///
/// Same as `load_env_files` but logs at info level to indicate a reload.
pub fn reload_env_files(config_dir: &Path) {
    load_env_files_inner(config_dir, true);
}

fn load_env_files_inner(config_dir: &Path, is_reload: bool) {
    let files = collect_env_files(config_dir);

    for path in &files {
        match dotenvy::from_path(path) {
            Ok(()) => debug!(path = %path.display(), "Loaded env file"),
            Err(err) => warn!(path = %path.display(), error = %err, "cannot load env file"),
        }
    }

    if is_reload && !files.is_empty() {
        info!(file_count = files.len(), "Secrets reloaded");
    }
}

fn collect_env_files(config_dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let base_env = config_dir.join(".env");
    if base_env.is_file() {
        files.push(base_env);
    }

    let pattern = config_dir.join(".*.env");
    if let Some(pattern_str) = pattern.to_str()
        && let Ok(paths) = glob(pattern_str)
    {
        files.extend(paths.filter_map(Result::ok));
    }

    files.sort();
    files
}

/// Returns true if the path is a `.env` or `.*.env` file.
pub fn is_env_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.') && name.ends_with(".env"))
}

/// Resolves a config value that may contain an environment variable reference.
///
/// If the value starts with `$`, treats the rest as an environment variable name
/// and returns its value. Otherwise returns the original value.
///
/// Returns `None` if the value is an env var reference but the variable is not set.
pub fn resolve(value: Option<String>) -> Option<String> {
    let value = value?;

    if let Some(var_name) = value.strip_prefix('$') {
        match std::env::var(var_name) {
            Ok(resolved) => Some(resolved),
            Err(_) => {
                warn!(var = %var_name, "Environment variable not set");
                None
            }
        }
    } else {
        Some(value)
    }
}

#[cfg(test)]
#[allow(unsafe_code)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn resolve_literal_value() {
        let result = resolve(Some(String::from("my-api-key")));
        assert_eq!(result, Some(String::from("my-api-key")));
    }

    #[test]
    fn resolve_env_var() {
        unsafe { std::env::set_var("TEST_SECRET_KEY", "secret123") };
        let result = resolve(Some(String::from("$TEST_SECRET_KEY")));
        assert_eq!(result, Some(String::from("secret123")));
        unsafe { std::env::remove_var("TEST_SECRET_KEY") };
    }

    #[test]
    fn resolve_missing_env_var() {
        unsafe { std::env::remove_var("NONEXISTENT_VAR") };
        let result = resolve(Some(String::from("$NONEXISTENT_VAR")));
        assert_eq!(result, None);
    }

    #[test]
    fn resolve_none() {
        let result = resolve(None);
        assert_eq!(result, None);
    }

    #[test]
    fn is_env_file_returns_true_for_dot_env() {
        assert!(is_env_file(Path::new("/config/.env")));
        assert!(is_env_file(Path::new(".env")));
    }

    #[test]
    fn is_env_file_returns_true_for_dot_prefixed_env() {
        assert!(is_env_file(Path::new("/config/.secrets.env")));
        assert!(is_env_file(Path::new(".weather.env")));
        assert!(is_env_file(Path::new("/home/user/.config/wayle/.api.env")));
    }

    #[test]
    fn is_env_file_returns_false_for_regular_file() {
        assert!(!is_env_file(Path::new("config.toml")));
        assert!(!is_env_file(Path::new("/path/to/file.txt")));
        assert!(!is_env_file(Path::new("README.md")));
    }

    #[test]
    fn is_env_file_returns_false_for_env_without_leading_dot() {
        assert!(!is_env_file(Path::new("secrets.env")));
        assert!(!is_env_file(Path::new("/config/weather.env")));
        assert!(!is_env_file(Path::new("my.env")));
    }
}
