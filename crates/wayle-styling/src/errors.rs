//! Error types for styling compilation.

use wayle_config::schemas::styling::ThemeProvider;

/// Errors that can occur during style compilation.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// SCSS compilation failed.
    #[error("scss compilation failed")]
    Compilation(#[from] Box<grass::Error>),

    /// File I/O error when writing variables.
    #[error("cannot write scss variables")]
    Io(#[from] std::io::Error),

    /// Theme provider not yet implemented.
    #[error("theme provider '{0}' is not yet implemented")]
    ProviderNotImplemented(ThemeProvider),
}
