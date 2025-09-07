use std::{any, path::PathBuf, time::Instant};

use toml::Value;

/// Represents a configuration change with path-based identification.
///
/// This struct captures all relevant information about a configuration change,
/// including what changed and when it changed.
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigChange {
    /// Path to the changed field using dot notation (e.g., "modules.clock.general.format").
    pub path: String,
    /// The previous value of the field, if available.
    pub old_value: Option<Value>,
    /// The new value of the field.
    pub new_value: Value,
    /// Timestamp when the change occurred.
    pub timestamp: Instant,
}

/// Errors that can occur during configuration operations.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// The specified configuration path does not exist.
    #[error("Invalid config path: {0}")]
    InvalidPath(String),

    /// The value type does not match the expected type for the field.
    #[error("Type mismatch at {path}: Expected {expected_type}, got {actual_value:?}")]
    TypeMismatch {
        /// The path where the type mismatch occurred.
        path: String,
        /// The expected type name.
        expected_type: &'static str,
        /// The actual value that was provided.
        actual_value: Value,
    },

    /// A configuration field that was previously available has been removed.
    #[error("Config field removed: {0}")]
    FieldRemoved(String),

    /// Error in path pattern matching or parsing
    #[error("invalid path pattern '{pattern}': {reason}")]
    PatternError {
        /// The pattern that failed to parse
        pattern: String,
        /// Reason why the pattern is invalid
        reason: String,
    },

    /// Error occurred while persisting configuration to disk
    #[error("failed to persist config to '{path}': {details}")]
    PersistenceError {
        /// Path where persistence failed
        path: PathBuf,
        /// Error details from the persistence operation
        details: String,
    },

    /// Error occurred while serializing configuration
    #[error("failed to serialize {content_type}: {details}")]
    SerializationError {
        /// Type of content being serialized (e.g., "config", "module settings")
        content_type: String,
        /// Serialization error details
        details: String,
    },

    /// Failed to parse TOML content
    #[error("failed to parse TOML from {location}: {details}")]
    TomlParseError {
        /// Location of the TOML (file path, "string", etc.)
        location: String,
        /// Parse error details
        details: String,
    },

    /// Failed to convert between config formats or types
    #[error("failed to convert {from} to {to}: {details}")]
    ConversionError {
        /// Source format/type
        from: String,
        /// Target format/type
        to: String,
        /// Conversion error details
        details: String,
    },

    /// Failed to initialize file watcher
    #[error("failed to initialize file watcher: {details}")]
    FileWatcherInitError {
        /// File watcher initialization error details
        details: String,
    },

    /// Error occurred while watching a specific file
    #[error("file watcher error for '{path}': {details}")]
    FileWatchError {
        /// Path being watched when error occurred
        path: PathBuf,
        /// File watcher error details
        details: String,
    },

    /// Error occurred during configuration processing or analysis
    #[error("config processing failed for '{operation}': {details}")]
    ProcessingError {
        /// The operation that was being processed
        operation: String,
        /// Processing error details
        details: String,
    },

    /// Error occurred during file I/O operations
    #[error("I/O error on '{path}': {details}")]
    IoError {
        /// Path where I/O error occurred
        path: PathBuf,
        /// I/O error details
        details: String,
    },

    /// Error occurred while acquiring locks
    #[error("failed to acquire {lock_type} lock: {details}")]
    LockError {
        /// Type of lock that failed (read, write)
        lock_type: String,
        /// Lock error details
        details: String,
    },

    /// A required service is unavailable
    #[error("{service} service unavailable: {details}")]
    ServiceUnavailable {
        /// Name of the service that is unavailable
        service: String,
        /// Details about why the service is unavailable
        details: String,
    },
}

impl ConfigChange {
    /// Creates a new configuration change.
    ///
    /// # Arguments
    ///
    /// * `path` - The dot-separated path to the configuration field
    /// * `old_value` - The previous value of the field (if known)
    /// * `new_value` - The new value of the field
    pub fn new(path: String, old_value: Option<Value>, new_value: Value) -> Self {
        Self {
            path,
            old_value,
            new_value,
            timestamp: Instant::now(),
        }
    }

    /// Extracts the new value as a specific type.
    ///
    /// This method attempts to deserialize the new value into the requested type.
    /// It provides type safety when extracting configuration values.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::TypeMismatch` if the value cannot be deserialized
    /// into the requested type.
    pub fn extract<T>(&self) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        let handle_err = |_e: toml::de::Error| -> ConfigError {
            ConfigError::TypeMismatch {
                path: self.path.clone(),
                expected_type: any::type_name::<T>(),
                actual_value: self.new_value.clone(),
            }
        };

        T::deserialize(self.new_value.clone()).map_err(handle_err)
    }

    /// Attempts to extract the new value as a string.
    ///
    /// Returns `None` if the value is not a string.
    pub fn as_string(&self) -> Option<String> {
        match &self.new_value {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    /// Extracts the new value as a string with a fallback default.
    ///
    /// If the value is not a string, returns the provided default value.
    pub fn as_string_or(&self, default: &str) -> String {
        self.as_string().unwrap_or_else(|| default.to_string())
    }
}
