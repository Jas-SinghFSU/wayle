//! Error types for styling compilation.

use wayle_config::schemas::styling::ThemeProvider;

/// Errors that can occur during style compilation.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// SCSS compilation failed.
    #[error("SCSS compilation failed: {0}")]
    Compilation(#[from] Box<grass::Error>),

    /// File I/O error when writing variables.
    #[error("Failed to write SCSS variables: {0}")]
    Io(#[from] std::io::Error),

    /// Theme provider palette resolution error.
    #[error("Failed to resolve theme provider's palette: {0:#?}")]
    ThemeProvider(String),

    /// Theme provider not yet implemented.
    #[error("Theme provider {0:?} is not yet implemented")]
    ProviderNotImplemented(ThemeProvider),
}
