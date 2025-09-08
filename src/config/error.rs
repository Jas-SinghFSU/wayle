use std::{
    fmt, io,
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Error types for the Wayle application.
///
/// This enum represents all possible errors that can occur during
/// configuration loading, parsing, and import operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration validation error
    #[error("configuration validation failed for '{component}': {details}")]
    ConfigValidation {
        /// Component that failed validation
        component: String,
        /// Validation error details
        details: String,
    },

    /// Configuration field missing or invalid
    #[error("invalid config field '{field}' in {component}: {reason}")]
    InvalidConfigField {
        /// The field that is invalid
        field: String,
        /// Component containing the field
        component: String,
        /// Reason why the field is invalid
        reason: String,
    },

    /// I/O operation error
    #[error("I/O error on '{path}': {details}")]
    IoError {
        /// Path where I/O error occurred
        path: PathBuf,
        /// I/O error details
        details: String,
    },

    /// Standard I/O operation error (for compatibility)
    #[error("IO error: {0:#?}")]
    Io(#[from] io::Error),

    /// TOML parsing error with location context
    #[error("failed to parse TOML at '{location}': {details}")]
    TomlParseError {
        /// Location of TOML being parsed (file path or "string")
        location: String,
        /// Parse error details
        details: String,
    },

    /// Import operation error with file context
    #[error("failed to import '{path}': {details}")]
    ImportError {
        /// Path of file being imported
        path: PathBuf,
        /// Import error details
        details: String,
    },
}

impl Error {
    /// Creates a TOML parsing error with optional file path context.
    ///
    /// # Arguments
    ///
    /// * `error` - The underlying parsing error
    /// * `path` - Optional path to the file that failed to parse
    pub fn toml_parse(error: impl fmt::Display, path: Option<&Path>) -> Self {
        let location = match path {
            Some(p) => {
                let clean_path = p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
                clean_path.to_string_lossy().to_string()
            }
            None => String::from("string"),
        };

        Error::TomlParseError {
            location,
            details: error.to_string(),
        }
    }

    /// Creates an import error with file path context.
    ///
    /// # Arguments
    ///
    /// * `error` - The underlying import error
    /// * `path` - Path to the file that failed to import
    pub fn import(error: impl fmt::Display, path: &Path) -> Self {
        let clean_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

        Error::ImportError {
            path: clean_path,
            details: error.to_string(),
        }
    }
}
