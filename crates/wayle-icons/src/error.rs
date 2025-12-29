use std::path::PathBuf;

use thiserror::Error;

/// Errors that can occur during icon operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Failed to fetch icon from CDN.
    #[error("failed to fetch icon '{slug}' from {icon_source}: {details}")]
    FetchError {
        /// The icon slug that failed to fetch.
        slug: String,
        /// The source name (e.g., "tabler", "simple-icons").
        icon_source: String,
        /// Error details from the HTTP request.
        details: String,
    },

    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Failed to write icon to disk.
    #[error("failed to write icon to '{path}': {details}")]
    WriteError {
        /// Path where the write failed.
        path: PathBuf,
        /// Error details.
        details: String,
    },

    /// Failed to delete icon.
    #[error("failed to delete icon '{name}': {details}")]
    DeleteError {
        /// Icon name that failed to delete.
        name: String,
        /// Error details.
        details: String,
    },

    /// Icon not found.
    #[error("icon '{name}' not found")]
    NotFound {
        /// Icon name that was not found.
        name: String,
    },

    /// Invalid icon source.
    #[error("unknown icon source '{name}', expected one of: tabler, simple-icons, lucide")]
    InvalidSource {
        /// The invalid source name provided.
        name: String,
    },

    /// Invalid SVG content.
    #[error("invalid SVG content for '{slug}': {details}")]
    InvalidSvg {
        /// The icon slug with invalid SVG.
        slug: String,
        /// Validation error details.
        details: String,
    },

    /// Failed to create icon directory.
    #[error("failed to create icon directory '{path}': {details}")]
    DirectoryError {
        /// Path where directory creation failed.
        path: PathBuf,
        /// Error details.
        details: String,
    },

    /// Failed to initialize icon registry with GTK.
    #[error("failed to initialize icon registry: {0}")]
    RegistryError(String),

    /// HOME environment variable not set.
    #[error("$HOME environment variable not set")]
    HomeNotSet,

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias for icon operations.
pub type Result<T> = std::result::Result<T, Error>;
