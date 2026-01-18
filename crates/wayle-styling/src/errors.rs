//! Error types for styling compilation.

use std::path::PathBuf;

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

    /// Palette file not found.
    #[error("palette file not found: {0}")]
    PaletteNotFound(PathBuf),

    /// Failed to parse palette JSON.
    #[error("cannot parse palette JSON")]
    PaletteJson(#[from] serde_json::Error),

    /// Theme provider not yet implemented.
    #[error("theme provider '{0}' is not yet implemented")]
    ProviderNotImplemented(ThemeProvider),
}
